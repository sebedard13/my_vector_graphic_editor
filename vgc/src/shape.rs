use crate::coord::{Coord, CoordPtr, CoordType};
use crate::curve;
use crate::curve::Curve;
use common::Rgba;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Shape {
    pub start: Rc<RefCell<Coord>>,
    pub(crate) curves: Vec<Curve>,
    pub color: Rgba,
}

impl Shape {
    pub fn toggle_separate_join_handle(&mut self, index: usize) {
        if self.is_handles_joined(index) {
            self.separate_handle(index);
        } else {
            self.join_handle(index);
        }
    }

    fn is_handles_joined(&self, index: usize) -> bool {
        let curve = &self.curves[index];
        curve.cp0 == curve.p1 || curve.cp1 == curve.p1
    }

    pub fn is_closed(&self) -> bool {
        if self.curves.is_empty() {
            return false;
        }
        let last_curve = self
            .curves
            .last()
            .expect("Shape should have at least one curve ");
        last_curve.p1 == self.start
    }

    pub fn close(&mut self) {
        if !self.is_closed() {
            let (_, _, _, p1) = self.get_coords_of_curve(self.curves.len() - 1);

            self.curves.push(Curve {
                cp0: p1,
                cp1: self.start.clone(),
                p1: self.start.clone(),
            });
        }
    }

    pub fn separate_handle(&mut self, curve_index_p1: usize) {
        let (coord_index0, coord_index1) = {
            let p0 = {
                if curve_index_p1 == 0 {
                    self.start.borrow()
                } else {
                    self.curves[curve_index_p1 - 1].p1.borrow()
                }
            };
            let current_curve = &self.curves[curve_index_p1];
            let cp0 = &current_curve.cp0.borrow();
            let cp1 = &current_curve.cp1.borrow();
            let p1 = &current_curve.p1.borrow();

            let next_curve = &self.curves[(curve_index_p1 + 1) % self.curves.len()];
            let cp2 = &next_curve.cp0.borrow();
            let cp3 = &next_curve.cp1.borrow();
            let p2 = &next_curve.p1.borrow();

            curve::tangent_cornor_pts(&p0, cp0, cp1, p1, cp2, cp3, p2)
        };

        self.curves[curve_index_p1].cp1 = Rc::new(RefCell::new(coord_index0));
        let len = self.curves.len();
        self.curves[(curve_index_p1 + 1) % len].cp0 = Rc::new(RefCell::new(coord_index1));
    }

    pub fn join_handle(&mut self, curve_index_p1: usize) {
        //cp0
        let coord_index = &self.curves[curve_index_p1].p1;
        let curve_after = (curve_index_p1 + 1) % self.curves.len();

        self.curves[curve_after].cp0 = coord_index.clone();

        //cp1
        let coord_index = &self.curves[curve_index_p1].p1;

        self.curves[curve_index_p1].cp1 = coord_index.clone();
    }

    pub fn to_path(&self) -> String {
        let mut path = String::new();
        let start = self.start.borrow();
        path.push_str(&format!("M {} {}", start.x, start.y));
        for curve in &self.curves {
            path.push(' ');
            path.push_str(&curve.to_path());
        }
        if self.is_closed() {
            path.push_str(" Z");
        }
        path
    }

    /// Visit each curve of the shape and call the visitor function with the curve index and 4 coords of the curve so p0, cp0, cp1, p1
    pub fn visit_full_curves(
        &self,
        mut visitor: impl FnMut(usize, &Coord, &Coord, &Coord, &Coord),
    ) {
        let start = self.start.borrow();
        let mut prev_coord = start;
        for (index, curve) in self.curves.iter().enumerate() {
            let cp0 = curve.cp0.borrow();
            let cp1 = curve.cp1.borrow();
            let p1 = curve.p1.borrow();

            visitor(index, &prev_coord, &cp0, &cp1, &p1);

            prev_coord = p1;
        }
    }

    /// Visit each curve and calculate the closest point on the curve to the coord
    ///
    /// Return (curve index, t value , distance, closest point)
    pub fn closest_curve(&self, coord: &Coord) -> (usize, f32, f32, Coord) {
        let mut min_distance = std::f32::MAX;
        let mut min_index = 0;
        let mut min_t = 0.0;
        let mut min_coord = self.start.borrow().clone();

        self.visit_full_curves(|curve_index, p0, cp0, cp1, p1| {
            let (t_min, distance, coord_closest) = curve::t_closest(coord, p0, cp0, cp1, p1);

            if distance < min_distance {
                min_distance = distance;
                min_index = curve_index;
                min_t = t_min;
                min_coord = coord_closest;
            }
        });
        (min_index, min_t, min_distance, min_coord)
    }

    pub fn get_coords_of_curve(
        &self,
        curve_index: usize,
    ) -> (CoordPtr, CoordPtr, CoordPtr, CoordPtr) {
        let mut prev_coord = self.start.clone();

        if curve_index > 0 {
            let prev_curve = self
                .curves
                .get(curve_index - 1)
                .expect("Index should be valid");
            prev_coord = prev_curve.p1.clone();
        }
        let curve = self.curves.get(curve_index).expect("Index should be valid");
        let cp0 = curve.cp0.clone();
        let cp1 = curve.cp1.clone();
        let p1 = curve.p1.clone();

        (prev_coord, cp0, cp1, p1)
    }

    pub fn push_coord(
        &mut self,
        cp0: Rc<RefCell<Coord>>,
        cp1: Rc<RefCell<Coord>>,
        p1: Rc<RefCell<Coord>>,
    ) {
        self.curves.push(Curve::new(cp0, cp1, p1));
    }

    pub fn get_coords_of_shape_tmp(&self) -> Vec<Rc<RefCell<Coord>>> {
        let mut vec = Vec::new();
        vec.push(self.start.clone());
        for curve in self.curves.iter() {
            vec.push(curve.cp0.clone());
            vec.push(curve.cp1.clone());
            vec.push(curve.p1.clone());
        }
        vec
    }

    pub fn move_coord(&mut self, coord_type: &CoordType, x: f32, y: f32) {
        match coord_type {
            CoordType::Start => {
                let mut coord = self.start.borrow_mut();
                coord.x = x;
                coord.y = y;
            }
            CoordType::Cp0(index_curve) => {
                let mut coord = self.curves[*index_curve].cp0.borrow_mut();
                coord.x = x;
                coord.y = y;
            }
            CoordType::Cp1(index_curve) => {
                let mut coord = self.curves[*index_curve].cp1.borrow_mut();
                coord.x = x;
                coord.y = y;
            }
            CoordType::P1(index_curve) => {
                let mut coord = self.curves[*index_curve].p1.borrow_mut();
                coord.x = x;
                coord.y = y;
            }
        }
    }

    /// Cut curve_index at t without chnaging the curve by replacing the handles
    pub fn insert_coord_smooth(&mut self, curve_index: usize, t: f32) {
        let (p0, cp0i, cp2i, p2) = self.get_coords_of_curve(curve_index);

        let (cp0, cp1l, p1, cp1r, cp2) = curve::add_smooth_result(
            &p0.borrow(),
            &cp0i.borrow(),
            &cp2i.borrow(),
            &p2.borrow(),
            t,
        );

        self.insert_coord_at(curve_index, p1);

        //for a straight line no handle
        if !(Rc::ptr_eq(&p0, &cp0i) && Rc::ptr_eq(&cp2i, &p2)) {
            self.curves[curve_index].cp1 = Rc::new(RefCell::new(cp1l));
            self.curves[curve_index + 1].cp0 = Rc::new(RefCell::new(cp1r));
        }

        //left has separate handle
        if !Rc::ptr_eq(&p0, &cp0i) {
            self.curves[curve_index].cp0 = Rc::new(RefCell::new(cp0));
        }

        //right has separate handle
        if !Rc::ptr_eq(&cp2i, &p2) {
            //Index valid because we just inserted
            self.curves[curve_index + 1].cp1 = Rc::new(RefCell::new(cp2));
        }
    }

    /// Cut curve_index with coord like cp0 coord coord coord cp1 p1
    pub fn insert_coord_at(&mut self, curve_index: usize, coord: Coord) {
        let p1 = Rc::new(RefCell::new(coord));

        let cp0 = self.curves[curve_index].cp0.clone();

        let new_curve = Curve::new(cp0, p1.clone(), p1.clone());

        self.curves[curve_index].cp0 = p1;

        self.curves.insert(curve_index, new_curve);
    }

    pub fn remove_start(&mut self) {}

    pub fn remove_curve(&mut self, curve_index: usize) {
        if self.is_closed() && self.curves.len() - 1 == curve_index {
            let curve = self.curves.remove(0);
            self.curves[curve_index - 1].cp1 = curve.cp1;
            self.curves[curve_index - 1].p1 = curve.p1.clone();
            self.start = curve.p1;
            return;
        }

        let cp0 = self.curves[curve_index].cp0.clone();

        self.curves.remove(curve_index);

        if let Some(curve) = self.curves.get_mut(curve_index) {
            curve.cp0 = cp0;
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.is_closed() {
            self.curves.len() == 1
        } else {
            self.curves.is_empty()
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::{coord::Coord, generate_from_push};

    #[test]
    fn cloest_pt() {
        let vgc = generate_from_push(vec![vec![
            Coord::new(0.43, 0.27),
            Coord::new(0.06577811, 0.2938202),
            Coord::new(0.0, 1.0),
            Coord::new(0.0, 1.0),
            Coord::new(0.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(0.7942219, 0.24617982),
            Coord::new(0.43, 0.27),
        ]]);

        let shape = vgc.get_shape(0).expect("Shape should exist");

        let (_, _, _, coord) = shape.closest_curve(&Coord::new(1.008, 0.612));
        assert_ne!(&coord.y, &1.0);
    }

    #[test]
    fn insert_coord_at() {
        let mut vgc = generate_from_push(vec![vec![
            Coord::new(0.0, 0.0),
            Coord::new(0.1, 0.1),
            Coord::new(0.9, 0.9),
            Coord::new(1.0, 1.0),
        ]]);

        let shape = vgc.get_shape_mut(0).expect("Shape should exist");

        shape.insert_coord_at(0, Coord::new(0.5, 0.5));

        let (p0, c0, c1l, p1) = shape.get_coords_of_curve(0);
        let (p1_2, c1r, c2, p2) = shape.get_coords_of_curve(1);

        assert_eq!(*p0.borrow(), Coord::new(0.0, 0.0));

        assert_eq!(*c0.borrow(), Coord::new(0.1, 0.1));
        assert_eq!(*c1l.borrow(), Coord::new(0.5, 0.5));
        assert_eq!(*p1.borrow(), Coord::new(0.5, 0.5));

        assert_eq!(*p1_2.borrow(), Coord::new(0.5, 0.5));

        assert_eq!(*c1r.borrow(), Coord::new(0.5, 0.5));

        assert_eq!(*c2.borrow(), Coord::new(0.9, 0.9));
        assert_eq!(*p2.borrow(), Coord::new(1.0, 1.0));

        assert_eq!(shape.curves.len(), 3); //Because close

        //+1 or +2 beacause of current borrow at 280 and 281
        assert_eq!(Rc::strong_count(&p0), 3 + 1); //Because close
        assert_eq!(Rc::strong_count(&c0), 1 + 1);
        assert_eq!(Rc::strong_count(&c1l), 3 + 4);
        assert_eq!(Rc::strong_count(&p1), 3 + 4);

        assert_eq!(Rc::strong_count(&p1_2), 3 + 4);
        assert_eq!(Rc::strong_count(&c1r), 3 + 4);

        assert_eq!(Rc::strong_count(&c2), 1 + 1);
        assert_eq!(Rc::strong_count(&p2), 2 + 1);
    }
}
