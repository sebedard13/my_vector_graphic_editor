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

use crate::{coord::CoordPtr, curve::add_smooth_result, curve2::intersection, shape::Shape};
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
    pub fn new(data: Vec<CoordOfIntersection>, start: usize) -> Self {
        Self { data, start }
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
        log::info!("A: {}", a.to_path());
        log::info!("B: {}", b.to_path());

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
    let start_a = result.len() - 1;

    intersections.sort();
    let mut iter = intersections.iter().enumerate();
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
            && current_intersection.unwrap().1.curve_index == curve_index
        {
            let intersection = current_intersection.unwrap().1;

            let t = (intersection.t - last_t) / (1.0 - last_t);

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

    GreinerShape::new(result, start_a)
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
    use common::{types::Coord, Rgba};

    use crate::shape::Shape;

    use super::CoordOfIntersection;

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

    //Function find_intersecions, create_shape and mark_entry_exit_points are tested more in detail in the tests of union.rs
}
