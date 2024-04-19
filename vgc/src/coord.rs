use common::types::Coord;
use common::Rgba;
use std::cell::{Ref, RefCell};
use std::fmt::Write;
use std::rc::Rc;

use crate::curve::cubic_bezier;
use crate::{curve, curve2, shape};

pub type CoordPtr = Rc<RefCell<Coord>>;

static ID_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

macro_rules! create_struct_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
        pub struct $name {
            id: usize,
        }

        impl $name {
            pub fn new() -> Self {
                $name { id: 0 }
            }

            pub fn new_with_id() -> Self {
                $name {
                    id: ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                }
            }

            pub fn new_id(&mut self) {
                self.id = ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    };
}

create_struct_id!(CoordId);
create_struct_id!(ShapeId);

#[derive(Debug, Clone)]
pub struct CoordRef {
    pub id: CoordId,
    pub coord: Coord,
}

impl CoordRef {
    pub fn new(coord: Coord) -> Self {
        CoordRef {
            id: CoordId::new(),
            coord,
        }
    }
}

pub struct ShapeRef {
    pub id: ShapeId,
    pub curves_path: Vec<CoordId>,
    pub color: Rgba,
}

pub struct CurveSolid<'a> {
    pub p0: &'a CoordRef,
    pub cp0: &'a CoordRef,
    pub cp1: &'a CoordRef,
    pub p1: &'a CoordRef,
}

impl<'a> CurveSolid<'a> {
    pub fn is_straight(&self) -> bool {
        self.p0.id == self.cp0.id && self.cp1.id == self.p1.id
    }

    pub fn right_handle_free(&self) -> bool {
        self.cp0.id != self.p0.id
    }

    pub fn left_handle_free(&self) -> bool {
        self.cp1.id != self.p1.id
    }
}

pub struct ShapeSolid {
    pub id: ShapeId,
    pub curves_path: Vec<CoordRef>,
    pub color: Rgba,
}

impl ShapeSolid {
    pub fn curve_len(&self) -> usize {
        (self.curves_path.len() - 1) / 3
    }

    pub fn get_curveSolid(&self, index: usize) -> Option<CurveSolid> {
        if index < self.curve_len() {
            let p0 = &self.curves_path[index * 3];
            let cp0 = &self.curves_path[(index * 3 + 1) % self.curves_path.len()];
            let cp1 = &self.curves_path[(index * 3 + 2) % self.curves_path.len()];
            let p1 = &self.curves_path[(index * 3 + 3) % self.curves_path.len()];
            return Some(CurveSolid { p0, cp0, cp1, p1 });
        }
        None
    }

    pub fn path(&self) -> String {
        let mut path = String::new();
        for i in 0..self.curves_path.len() {
            let coord = self.curves_path[i].coord;
            if i == 0 {
                write!(&mut path, "M {} {} ", coord.x(), coord.y()).expect("Write should be ok");
            } else if (i - 1) % 3 == 0 {
                write!(&mut path, "C {} {} ", coord.x(), coord.y()).expect("Write should be ok");
            } else {
                write!(&mut path, "{} {} ", coord.x(), coord.y()).expect("Write should be ok");
            }
        }
        write!(&mut path, "Z").expect("Write should be ok");
        path
    }

    pub fn toggle_separate_join_handle(&mut self, index: usize) {
        if self.is_handles_joined(index) {
            self.separate_handle(index);
        } else {
            self.join_handle(index);
        }
    }

    fn is_handles_joined(&self, index: usize) -> bool {
        let curve = self.get_curveSolid(index).expect("Curve should exist");
        curve.cp0.id == curve.p1.id || curve.cp1.id == curve.p1.id
    }

    pub fn join_handle(&mut self, curve_index_p1: usize) {
        let index_p1 = (curve_index_p1 * 3 + 3) % self.curves_path.len();
        let len = self.curves_path.len();
        self.curves_path[(index_p1 - 1) % len] = self.curves_path[index_p1].clone();
        self.curves_path[(index_p1 + 1) % len] = self.curves_path[index_p1].clone();
    }

    pub fn separate_handle(&mut self, curve_index_p1: usize) {
        let index_p1 = (curve_index_p1 * 3 + 3) % self.curves_path.len();
        let (coord_index0, coord_index1) = {
            let p0 = &self.curves_path[(index_p1 - 3) % self.curves_path.len()];
            let cp0 = &self.curves_path[(index_p1 - 2) % self.curves_path.len()];
            let cp1 = &self.curves_path[(index_p1 - 1) % self.curves_path.len()];
            let p1 = &self.curves_path[index_p1];

            let cp2 = &self.curves_path[(index_p1 + 1) % self.curves_path.len()];
            let cp3 = &self.curves_path[(index_p1 + 2) % self.curves_path.len()];
            let p2 = &self.curves_path[(index_p1 + 3) % self.curves_path.len()];

            curve::tangent_cornor_pts(
                &p0.coord, &cp0.coord, &cp1.coord, &p1.coord, &cp2.coord, &cp3.coord, &p2.coord,
            )
        };

        let len = self.curves_path.len();
        self.curves_path[(index_p1 - 1) % len] = CoordRef::new(coord_index0);
        self.curves_path[(index_p1 + 1) % len] = CoordRef::new(coord_index1);
    }

    /// Visit each curve and calculate the closest point on the curve to the coord
    ///
    /// Return (curve index, t value , distance, closest point)
    pub fn closest_curve(&self, coord: &Coord) -> (usize, f32, f32, Coord) {
        let mut min_distance = std::f32::MAX;
        let mut min_index = 0;
        let mut min_t = 0.0;
        let mut min_coord = Coord::new(-1000.0, -1000.0);

        for curve_index in 0..self.curve_len() {
            let curve = self
                .get_curveSolid(curve_index)
                .expect("Curve should exist");
            let (t_min, distance, coord_closest) = curve::t_closest(
                coord,
                &curve.p0.coord,
                &curve.cp0.coord,
                &curve.cp1.coord,
                &curve.p1.coord,
            );

            if distance < min_distance {
                min_distance = distance;
                min_index = curve_index;
                min_t = t_min;
                min_coord = coord_closest;
            }
        }
        (min_index, min_t, min_distance, min_coord)
    }

    /// Cut curve_index at t without chnaging the curve by replacing the handles
    pub fn insert_coord_smooth(&mut self, curve_index: usize, t: f32) {
        let curve = self
            .get_curveSolid(curve_index)
            .expect("Curve should exist");

        let (cp0, cp1l, p1, cp1r, cp2) = curve::add_smooth_result(
            &curve.p0.coord,
            &curve.cp0.coord,
            &curve.cp1.coord,
            &curve.p1.coord,
            t,
        );

        let cp1l = CoordRef::new(cp1l);
        let p1 = CoordRef::new(p1);
        let cp1r = CoordRef::new(cp1r);

        let index_cp1 = (curve_index * 3 + 2) % self.curves_path.len();
        //for a straight line no handle
        if !(curve.is_straight()) {
            let new_coords = vec![cp1l, p1, cp1r];
            self.curves_path.splice(index_cp1..index_cp1, new_coords);
        }
        //left has separate handle
        else if !curve.left_handle_free() {
            let new_coords = vec![cp1l, p1.clone(), p1];
            self.curves_path.splice(index_cp1..index_cp1, new_coords);
        }
        //right has separate handle
        else if !curve.right_handle_free() {
            let new_coords = vec![p1.clone(), p1, cp1r];
            self.curves_path.splice(index_cp1..index_cp1, new_coords);
        }
    }

    pub fn insert_coord(&mut self, curve_index: usize, coord: Coord) {
        let p1 = CoordRef::new(coord);
        let new_coords = vec![p1.clone(), p1.clone(), p1];
        let index_cp1 = (curve_index * 3 + 2) % self.curves_path.len();
        self.curves_path.splice(index_cp1..index_cp1, new_coords);
    }

    pub fn remove_curve(&mut self, curve_index: usize) {
        let index_p1 = (curve_index * 3 + 3) % self.curves_path.len();

        self.curves_path.splice(index_p1 - 1..index_p1 + 1, []);
    }

    /// Return true if the coord is inside the shape
    /// Use the even-odd rule
    pub fn contains(&self, coord: &Coord) -> bool {
        let mut count = 0;
        for curve_index in 0..self.curve_len() {
            let curve = self
                .get_curveSolid(curve_index)
                .expect("Curve should exist");
            let p0 = curve.p0.coord;
            let cp0 = curve.cp0.coord;
            let cp1 = curve.cp1.coord;
            let p1 = curve.p1.coord;

            let t_intersections = curve2::intersection_with_y(&p0, &cp0, &cp1, &p1, coord.y());
            for t in t_intersections {
                let x = cubic_bezier(t, &p0, &cp0, &cp1, &p1).x();
                if x > coord.x() {
                    count += 1;
                }
            }
        }
        count % 2 == 1
    }
}

use common::pures::{Affine, Vec2};

impl ShapeSolid {
    pub fn new() -> Self {
        ShapeSolid {
            id: ShapeId::new(),
            curves_path: Vec::new(),
            color: Rgba::black(),
        }
    }

    //List of coordinates of curves. The first coordinate is the start of the curve.
    pub fn new_from_path(coords: &Vec<Coord>, transform: Affine) -> Self {
        let mut shape = ShapeSolid::new();

        for i in 0..coords.len() {
            shape
                .curves_path
                .push(CoordRef::new(coords[i].transform(&transform)));
        }

        shape
    }

    pub fn new_circle(center: Coord, radius: Vec2) -> Self {
        let transform = Affine::identity().scale(radius).translate(center.c);

        //https://spencermortensen.com/articles/bezier-circle/
        let a = 1.000_055_19;
        let b = 0.553_426_86;
        let c = 0.998_735_85;

        let coords = vec![
            Coord::new(0.0, a),
            Coord::new(b, c),
            Coord::new(c, b),
            Coord::new(a, 0.0),
            Coord::new(c, -b),
            Coord::new(b, -c),
            Coord::new(0.0, -a),
            Coord::new(-b, -c),
            Coord::new(-c, -b),
            Coord::new(-a, 0.0),
            Coord::new(-c, b),
            Coord::new(-b, c),
            Coord::new(0.0, a),
        ];

        ShapeSolid::new_from_path(&coords, transform)
    }
}

pub struct VgcDataBase {
    pub background: Rgba,
    pub shapes: Vec<ShapeRef>,
    pub coords: Vec<CoordRef>,
}

impl VgcDataBase {
    pub fn shape_select(&self, index: ShapeId) -> Option<ShapeSolid> {
        if let Some(shape) = self.shapes.iter().find(|s| s.id == index) {
            let mut curves_path = Vec::new();
            for id in shape.curves_path.iter() {
                if let Some(coord) = self.coords_select(*id) {
                    curves_path.push(coord);
                }
            }
            return Some(ShapeSolid {
                id: shape.id,
                curves_path,
                color: shape.color.clone(),
            });
        }
        None
    }

    pub fn coords_select(&self, index: CoordId) -> Option<CoordRef> {
        if let Some(coord) = self.coords.iter().find(|c| c.id == index) {
            return Some(CoordRef {
                id: coord.id,
                coord: coord.coord,
            });
        }
        None
    }

    pub fn coords_update(&mut self, mut updated_coords: Vec<CoordRef>) -> Vec<CoordRef> {
        let mut rtn_coords = Vec::with_capacity(updated_coords.len());
        for updated_coord in &mut updated_coords {
            if updated_coord.id == CoordId::new() {
                updated_coord.id.new_id();
                self.coords.push(updated_coord.clone());
                rtn_coords.push(updated_coord.clone());
            } else {
                if let Some(coord) = self.coords.iter_mut().find(|c| c.id == updated_coord.id) {
                    *coord = updated_coord.clone();
                    rtn_coords.push(updated_coord.clone());
                } else {
                    panic!("Coord should exist if id is not new")
                }
            }
        }
        rtn_coords
    }

    pub fn coords_delete(&mut self, index: Vec<CoordRef>) {
        for coord in index {
            self.coords.retain(|c| c.id != coord.id);
        }
    }

    pub fn coords_delete_cascade(&mut self, index: Vec<CoordRef>) {
        for coord in index {
            self.coords.retain(|c| c.id != coord.id);
            for shape in &mut self.shapes {
                shape.curves_path.retain(|c| *c != coord.id);
            }
        }
    }

    pub fn shape_insert_coords_between(
        &mut self,
        coords: Vec<CoordId>,
        coord_a: CoordId,
        coord_b: CoordId,
    ) {
        let mut removed_coords: Vec<CoordId> = vec![];
        for shape in &mut self.shapes {
            let index_a = shape.curves_path.iter().position(|c| *c == coord_a);
            let index_b = shape.curves_path.iter().position(|c| *c == coord_b);
            if index_a.is_none() && index_b.is_none() {
                continue;
            } else if index_a.is_some() && index_b.is_none()
                || index_a.is_none() && index_b.is_some()
            {
                panic!("Both coords should exist in the shape")
            } else {
                let index_a = index_a.unwrap();
                let index_b = index_b.unwrap();
                let min = index_a.min(index_b);
                let max = index_a.max(index_b);
                let rmv = shape.curves_path.splice(min..max, coords.clone());
                let mut iter: Vec<CoordId> = rmv.collect();
                removed_coords.append(&mut iter);
            }
        }
    }

    pub fn shape_diff_apply(&mut self, new_shape: ShapeSolid) {
        let shape_index = self
            .shapes
            .iter()
            .position(|s| s.id == new_shape.id)
            .expect("Valid shape id");

        let old_shape = &mut self.shapes[shape_index];
        let new_i = 0;
        let old_i = 0;

        if old_shape.curves_path[0] != new_shape.curves_path[0].id {
           panic!("First coord should be the same")
        }

        while true {
            
        }
    }
}
