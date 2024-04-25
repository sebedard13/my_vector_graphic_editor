/*
Implementation of boolean operations on shapes.
For Shape A and B
Union : A OR B
Intersection : A AND B
Difference : A NOT B
*/

mod difference;
mod intersection;
mod union;

use crate::{
    coord::CoordPtr,
    curve::{add_smooth_result, is_line},
    curve2::intersection,
    shape::Shape,
};
use common::{dbg_str, types::Coord};
use std::{cell::RefCell, rc::Rc};

pub use self::{difference::ShapeDifference, intersection::ShapeIntersection, union::ShapeUnion};

impl Shape {
    pub fn union(&self, other: &Shape) -> ShapeUnion {
        union::shape_union(self, other)
    }

    pub fn intersection(&self, other: &Shape) -> ShapeIntersection {
        intersection::shape_intersection(self, other)
    }

    pub fn difference(&self, other: &Shape) -> ShapeDifference {
        difference::shape_difference(self, other)
    }
}

/// When calculating the union of two shapes, we need to find all the intersection points between the two shapes.
/// GreinerShape is a representation of a shape where all intersection points are added as separate coordinates and marked as such.
/// It contains a doubly linked list of CoordOfIntersection.
struct GreinerShape {
    pub data: Vec<CoordOfIntersection>,
    pub start: usize,
    pub intersections_len: usize,
}

#[derive(Clone)]
struct CoordOfIntersection {
    pub curve_index: usize,
    pub t: f32,
    pub neighbor: Option<usize>,
    pub next: Option<usize>,
    pub prev: Option<usize>,
    pub entry: bool,
    pub intersect: bool,
    pub coord: Coord,
    pub rel_coord: Option<CoordPtr>,
}

impl GreinerShape {
    pub fn new(data: Vec<CoordOfIntersection>, start: usize, intersections_len: usize) -> Self {
        Self {
            data,
            start,
            intersections_len,
        }
    }

    #[allow(dead_code)] // For testing
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[allow(dead_code)] // For testing
    pub fn get(&self, index: usize) -> &CoordOfIntersection {
        let mut current = self.start;
        let mut count = 0;
        while count < index && self.data[current].next.is_some() {
            current = self.data[current].next.unwrap();
            count += 1;
        }
        &self.data[current]
    }

    #[allow(dead_code)] // For testing
    pub fn print_coords_table(&self) {
        println!(
            "Start: {} | Intersections: {}",
            self.start, self.intersections_len
        );
        println!("Index, x, y, In.sect, Next, Prev");
        for (i, c) in self.data.iter().enumerate() {
            let x = format!("{:.2}", c.coord.x());
            let y = format!("{:.2}", c.coord.y());

            println!(
                "{:>7}, {:>7}, {:>7}, {:>7}, {:>7}, {:>7},{:?}",
                i,
                x,
                y,
                c.intersect,
                c.next.unwrap(),
                c.prev.unwrap(),
                c.neighbor
            );
        }
    }
}

impl CoordOfIntersection {
    pub fn from_existing(rel_coord: &CoordPtr) -> Self {
        Self {
            curve_index: 0, //we don't need this
            t: 0.0,         //we don't need this
            neighbor: None,
            next: None,
            prev: None,
            entry: false,
            intersect: false,
            coord: rel_coord.borrow().clone(),
            rel_coord: Some(rel_coord.clone()),
        }
    }

    pub fn from_intersection(coord: Coord, t: f32, curve_index: usize) -> Self {
        Self {
            curve_index,
            t,
            neighbor: None,
            next: None,
            prev: None,
            entry: false,
            intersect: true,
            coord: coord,
            rel_coord: None,
        }
    }

    pub fn from_new(coord: Coord) -> Self {
        Self {
            curve_index: 0, //we don't need this
            t: 0.0,         //we don't need this
            neighbor: None,
            next: None,
            prev: None,
            entry: false,
            intersect: false,
            coord: coord,
            rel_coord: None,
        }
    }

    pub fn coord_ptr(&self) -> CoordPtr {
        match &self.rel_coord {
            Some(rel_coord) => rel_coord.clone(),
            None => Rc::new(RefCell::new(self.coord.clone())),
        }
    }
}

impl Ord for CoordOfIntersection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord_curve = self.curve_index.cmp(&other.curve_index);
        if ord_curve == std::cmp::Ordering::Equal {
            self.t.partial_cmp(&other.t).expect("Should be a number")
        } else {
            ord_curve
        }
    }
}

impl Eq for CoordOfIntersection {}

impl PartialOrd for CoordOfIntersection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CoordOfIntersection {
    fn eq(&self, other: &Self) -> bool {
        self.curve_index == other.curve_index && self.t == other.t
    }
}

fn find_intersecions(a: &Shape, b: &Shape) -> (Vec<CoordOfIntersection>, Vec<CoordOfIntersection>) {
    let mut intersections_a: Vec<CoordOfIntersection> = Vec::with_capacity(a.curves.len());
    let mut intersections_b: Vec<CoordOfIntersection> = Vec::with_capacity(b.curves.len());

    for i in 0..a.curves.len() {
        let (a_p0, a_cp0, a_cp1, a_p1) = a.get_coords_of_curve(i);

        for j in 0..b.curves.len() {
            let (b_p0, b_cp0, b_cp1, b_p1) = b.get_coords_of_curve(j);

            let intersection_points = intersection(
                &a_p0.borrow(),
                &a_cp0.borrow(),
                &a_cp1.borrow(),
                &a_p1.borrow(),
                &b_p0.borrow(),
                &b_cp0.borrow(),
                &b_cp1.borrow(),
                &b_p1.borrow(),
            );

            for point in intersection_points {
                let mut point_a = CoordOfIntersection::from_intersection(point.coord, point.t1, i);

                let mut point_b = CoordOfIntersection::from_intersection(point.coord, point.t2, j);
                if point_a.t == 1.0 {
                    point_a.t = 0.0;
                    point_a.curve_index = (point_a.curve_index + 1) % a.curves.len();
                    point_a.rel_coord = Some(a_p1.clone());
                    point_b.rel_coord = Some(a_p1.clone());
                } else if point_a.t == 0.0 {
                    point_a.rel_coord = Some(a_p0.clone());
                    point_b.rel_coord = Some(a_p0.clone());
                }
                if point_b.t == 1.0 {
                    point_b.t = 0.0;
                    point_b.curve_index = (point_b.curve_index + 1) % b.curves.len();
                    point_b.rel_coord = Some(b_p1.clone());
                    point_a.rel_coord = Some(b_p1.clone());
                } else if point_b.t == 0.0 {
                    point_b.rel_coord = Some(b_p0.clone());
                    point_a.rel_coord = Some(b_p0.clone());
                }

                point_a.neighbor = Some(intersections_b.len());
                point_b.neighbor = Some(intersections_a.len());

                intersections_a.push(point_a);
                intersections_b.push(point_b);
            }
        }
    }

    assert_intersections_even_count(a, &mut intersections_a, b, &mut intersections_b);

    (intersections_a, intersections_b)
}

/// A Shape should be closed so we should have an even number of intersections between two shapes.
/// If not, it is maybe a bug in the intersection calculation or a precision problem.
/// In this case, we remove the closest intersection point to the others and update the neighbor index.
fn assert_intersections_even_count(
    a: &Shape,
    intersections_a: &mut Vec<CoordOfIntersection>,
    b: &Shape,
    intersections_b: &mut Vec<CoordOfIntersection>,
) {
    assert_eq!(intersections_a.len(), intersections_b.len());
    if (intersections_a.len() % 2) != 0 {
        log::warn!(
            "{}",
            dbg_str!("Shape are closed so we should have an even number of intersections. Fix will be applied with a lost in precision")
        );
        log::info!("A: {}", a.path());
        log::info!("B: {}", b.path());

        let mut difference = Vec::with_capacity(intersections_a.len() * intersections_a.len());
        for i in 0..intersections_a.len() {
            for j in 0..intersections_a.len() {
                difference.push((intersections_a[i].coord - intersections_a[j].coord).norm());
            }
        }

        //find min of difference
        let mut min = f32::MAX;
        let mut min_index: usize = 0;
        for i in 0..difference.len() {
            if difference[i] < min {
                min = difference[i];
                min_index = (i - (i % intersections_a.len())) / intersections_a.len();
            }
        }

        intersections_a.remove(min_index);
        intersections_b.remove(min_index);

        for i in 0..intersections_a.len() {
            if intersections_a[i].neighbor.unwrap() > min_index {
                intersections_a[i].neighbor = Some(intersections_a[i].neighbor.unwrap() - 1);
                intersections_b[i].neighbor = Some(intersections_b[i].neighbor.unwrap() - 1);
            }
        }
    }
    assert_eq!(intersections_a.len() % 2, 0); // Shape are closed so we should have an even number of intersections
}

fn create_shape(shape: &Shape, mut intersections: Vec<CoordOfIntersection>) -> GreinerShape {
    let mut result = Vec::with_capacity(shape.curves.len() * 3 + intersections.len() * 3);
    result.append(&mut intersections.clone());
    result.push(CoordOfIntersection::from_existing(&shape.start));

    let mut current_index = result.len() - 1;
    let mut start_a = result.len() - 1;

    intersections.sort();
    let mut iter = intersections.iter();
    let mut current_intersection = iter.next();

    for curve_index in 0..shape.curves.len() {
        let (a_p0, a_cp0, a_cp1, a_p1) = shape.get_coords_of_curve(curve_index);
        let coord = *a_p0.borrow();
        let mut current_p0 = (coord, Some(a_p0));
        let coord = *a_cp0.borrow();
        let mut current_cp0 = (coord, Some(a_cp0));
        let coord = *a_cp1.borrow();
        let mut current_cp1 = (coord, Some(a_cp1));
        let coord = *a_p1.borrow();
        let current_p1 = (coord, a_p1);

        let mut last_t = 0.0;
        while current_intersection.is_some()
            && current_intersection.unwrap().curve_index == curve_index
        {
            let intersection = current_intersection.unwrap();

            let t = (intersection.t - last_t) / (1.0 - last_t);

            // with t ==0.0, the intersection is at the start of the curve so we can remove the last p1 added
            if t == 0.0 {
                //Neighbor is actualy the index of itself because they are not sorted in result
                let index_intersection = intersection.neighbor.unwrap();
                if result[current_index].prev.is_some() {
                    let prev_index = result[current_index].prev.unwrap();

                    result[prev_index].next = Some(index_intersection);
                    result[index_intersection].prev = Some(prev_index);
                }

                result.remove(current_index);

                if current_index == start_a {
                    start_a = index_intersection;
                }

                current_index = index_intersection;
                current_intersection = iter.next();
                continue;
            }

            let (new_cp0, new_cp1l, new_p1, new_cp1r, new_cp2) = add_smooth_result(
                &current_p0.0,
                &current_cp0.0,
                &current_cp1.0,
                &current_p1.0,
                t,
            );

            last_t = intersection.t;

            let mut cp0 = CoordOfIntersection::from_new(new_cp0);
            result[current_index].next = Some(result.len());
            cp0.prev = Some(current_index);
            result.push(cp0);
            current_index = result.len() - 1;

            let mut cp1l = CoordOfIntersection::from_new(new_cp1l);
            result[current_index].next = Some(result.len());
            cp1l.prev = Some(current_index);
            result.push(cp1l);
            current_index = result.len() - 1;

            //Neighbor is actualy the index of itself because they are not sorted in result
            let index_intersection = intersection.neighbor.unwrap();
            result[current_index].next = Some(index_intersection);
            result[index_intersection].prev = Some(current_index);
            current_index = index_intersection;

            current_p0 = (new_p1, None);
            current_cp0 = (new_cp1r, None);
            current_cp1 = (new_cp2, None);

            current_intersection = iter.next();
        }

        match current_cp0.1 {
            Some(cp0) => {
                let mut cp0 = CoordOfIntersection::from_existing(&cp0);
                result[current_index].next = Some(result.len());
                cp0.prev = Some(current_index);
                result.push(cp0);
                current_index = result.len() - 1;
            }
            None => {
                let mut cp0 = CoordOfIntersection::from_new(current_cp0.0);
                result[current_index].next = Some(result.len());
                cp0.prev = Some(current_index);
                result.push(cp0);
                current_index = result.len() - 1;
            }
        }

        match current_cp1.1 {
            Some(cp1) => {
                let mut cp1 = CoordOfIntersection::from_existing(&cp1);
                result[current_index].next = Some(result.len());
                cp1.prev = Some(current_index);
                result.push(cp1);
                current_index = result.len() - 1;
            }
            None => {
                let mut cp1 = CoordOfIntersection::from_new(current_cp1.0);
                result[current_index].next = Some(result.len());
                cp1.prev = Some(current_index);
                result.push(cp1);
                current_index = result.len() - 1;
            }
        }

        let mut p1 = CoordOfIntersection::from_existing(&current_p1.1);
        result[current_index].next = Some(result.len());
        p1.prev = Some(current_index);
        result.push(p1);
        current_index = result.len() - 1;
    }

    result.pop(); //Remove the last point which is the start point
    let var_name = result.len() - 1;
    let last_cp = &mut result[var_name];
    last_cp.next = Some(start_a);
    result[start_a].prev = Some(result.len() - 1);

    compress_coord_ptr(&mut result, start_a);

    GreinerShape::new(result, start_a, intersections.len())
}

fn compress_coord_ptr(list: &mut Vec<CoordOfIntersection>, start_a: usize) {
    let mut index = 0;
    let mut current_index = start_a;

    let mut i_p0 = start_a;
    let mut i_cp0 = 0;
    let mut i_cp1 = 0;

    while index == 0 || current_index != start_a {
        //cp0
        if index % 3 == 1 {
            i_cp0 = current_index;
        }
        //cp1
        else if index % 3 == 2 {
            i_cp1 = current_index;
        }
        //p1
        if index % 3 == 0 && index != 0 {
            let i_p1 = current_index;

            let p0 = list[i_p0].coord;
            let cp0 = list[i_cp0].coord;
            let cp1 = list[i_cp1].coord;
            let p1 = list[i_p1].coord;

            if is_line(&p0, &cp0, &cp1, &p1) {
                list[i_cp0].coord = list[i_p0].coord;
                list[i_cp1].coord = list[i_p1].coord;

                if list[i_p0].rel_coord.is_some() {
                    list[i_cp0].rel_coord = list[i_p0].rel_coord.clone();
                } else {
                    let rc = Some(list[i_p0].coord_ptr());
                    list[i_p0].rel_coord = rc.clone();
                    list[i_cp0].rel_coord = rc;
                }
                if list[i_p1].rel_coord.is_some() {
                    list[i_cp1].rel_coord = list[i_p1].rel_coord.clone();
                } else {
                    let rc = Some(list[i_p1].coord_ptr());
                    list[i_p1].rel_coord = rc.clone();
                    list[i_cp1].rel_coord = rc;
                }
            }

            i_p0 = current_index;
        }
        current_index = list[current_index].next.unwrap();
        index += 1;
    }
}

fn mark_entry_exit_points(ag: &mut GreinerShape, a: &Shape, bg: &mut GreinerShape, b: &Shape) {
    let mut status_entry = true;
    let coord = &ag.data[ag.start].coord;
    let con = b.contains(coord);
    if con {
        status_entry = false;
    }

    let mut current_index = ag.start;
    while ag.data[current_index].next.is_some() && ag.data[current_index].next.unwrap() != ag.start
    {
        let next_index = ag.data[current_index].next.unwrap();
        let next = &mut ag.data[next_index];
        if next.intersect {
            next.entry = status_entry;
            status_entry = !status_entry;
        }
        current_index = next_index;
    }

    status_entry = true;
    if a.contains(&bg.data[bg.start].coord) {
        status_entry = false;
    }

    let mut current_index = bg.start;
    while bg.data[current_index].next.is_some() && bg.data[current_index].next.unwrap() != bg.start
    {
        let next_index = bg.data[current_index].next.unwrap();
        let next = &mut bg.data[next_index];
        if next.intersect {
            next.entry = status_entry;
            status_entry = !status_entry;
        }
        current_index = next_index;
    }
}

#[cfg(test)]
mod test {
    use common::{pures::Affine, types::Coord, Rgba};

    use crate::shape::Shape;

    use super::{find_intersecions, CoordOfIntersection};

    #[test]
    fn given_intersection_count_not_even_when_assert_then_is_fix() {
        let a = Shape::new(Coord::new(0.0, 0.0), Rgba::new(255, 255, 255, 255));
        let b = Shape::new(Coord::new(0.0, 0.0), Rgba::new(255, 255, 255, 255));

        let mut intersection_a = vec![
            CoordOfIntersection::from_intersection(Coord::new(0.0, 0.0), 0.0, 0),
            CoordOfIntersection::from_intersection(Coord::new(0.00001, 0.000001), 0.000001, 0),
            CoordOfIntersection::from_intersection(Coord::new(2.0, 1.0), 0.0, 1),
        ];

        let mut intersection_b = vec![
            CoordOfIntersection::from_intersection(Coord::new(0.0, 0.0), 0.0, 0),
            CoordOfIntersection::from_intersection(Coord::new(0.00001, 0.000001), 0.000001, 0),
            CoordOfIntersection::from_intersection(Coord::new(2.0, 1.0), 0.0, 1),
        ];

        for i in 0..intersection_a.len() {
            intersection_a[i].neighbor = Some(i);
            intersection_b[i].neighbor = Some(i);
        }

        super::assert_intersections_even_count(&a, &mut intersection_a, &b, &mut intersection_b);

        assert_eq!(intersection_a.len(), 2);
        assert_eq!(intersection_b.len(), 2);

        for i in 0..intersection_a.len() {
            assert_eq!(intersection_a[i].neighbor.unwrap(), i);
            assert_eq!(intersection_b[i].neighbor.unwrap(), i);
        }
    }

    #[test]
    fn given_bug_diff_when_difference() {
        // A: M 1 -1 C 1 -1 1 1 1 1 C 1 1 0 0 0 0 C 0 0 -1 -1 -1 -1 C -1 -1 1 -1 1 -1 Z wasm_vgc_bg.js:1982:12
        //B:  C -0.5066913 -0.54993117 -0.50866926 -0.53057325 -0.5123266 -0.5123266 Z

        let coords_a = vec![
            Coord::new(1.0, -1.0),
            Coord::new(1.0, -1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0),
            Coord::new(-1.0, -1.0),
            Coord::new(-1.0, -1.0),
            Coord::new(-1.0, -1.0),
            Coord::new(1.0, -1.0),
            Coord::new(1.0, -1.0),
        ];

        let coords_b = vec![
            Coord::new(-0.5123266, -0.5123266),
            Coord::new(-0.25828606, -0.25828606),
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(-1.0, 1.0),
            Coord::new(-1.0, 1.0),
            Coord::new(-1.0, 1.0),
            Coord::new(-1.0, -1.0),
            Coord::new(-1.0, -1.0),
            Coord::new(-1.0, -1.0),
            Coord::new(-0.880739, -0.880739),
            Coord::new(-0.7245886, -0.7245886),
            Coord::new(-0.7016079, -0.75289893),
            Coord::new(-0.67217433, -0.7698959),
            Coord::new(-0.6400001, -0.77001095),
            Coord::new(-0.56620985, -0.76974714),
            Coord::new(-0.50683534, -0.6806853),
            Coord::new(-0.5066594, -0.56999993),
            Coord::new(-0.5066913, -0.54993117),
            Coord::new(-0.50866926, -0.53057325),
            Coord::new(-0.5123266, -0.5123266),
        ];

        let a = Shape::new_from_path(&coords_a, Affine::identity(), Rgba::white());
        let b = Shape::new_from_path(&coords_b, Affine::identity(), Rgba::white());

        let merged = find_intersecions(&a, &b);
        assert_eq!(merged.0.len(), 2);
        assert_eq!(merged.0[0].t, 0.654658138);
        assert_eq!(merged.0[1].t, 0.508218467);
    }

    //Function find_intersecions, create_shape and mark_entry_exit_points are tested more in detail in the tests of union.rs
}
