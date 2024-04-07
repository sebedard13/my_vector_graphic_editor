/*
Implementation of boolean operations on shapes.
For Shape A and B
Union : A OR B
Intersection : A AND B
Difference : A NOR B
*/

use std::{cell::RefCell, rc::Rc};

use common::types::Coord;

use crate::{
    coord::CoordPtr,
    curve::{add_smooth_result, Curve},
    curve2::intersection,
    shape::Shape,
};

// When calculating the union of two shapes, we need to find all the intersection points between the two shapes.
// GreinerShape is a representation of a shape where all intersection points are added as separate coordinates and marked as such.
// It contains a doubly linked list of CoordOfIntersection.
struct GreinerShape {
    pub data: Vec<CoordOfIntersection>,
    pub start: usize,
    pub intersection_count: usize,
}

impl GreinerShape {
    pub fn new(data: Vec<CoordOfIntersection>, start: usize, intersection_count: usize) -> Self {
        Self {
            data,
            start,
            intersection_count,
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
        println!("Index, x, yCoordinate, Intersect, Next, Prev");
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

#[allow(dead_code)]
fn find_all_intersecion(a: &Shape, b: &Shape) -> (GreinerShape, GreinerShape) {
    let mut intersection_a: Vec<CoordOfIntersection> = Vec::with_capacity(a.curves.len());
    let mut intersection_b: Vec<CoordOfIntersection> = Vec::with_capacity(b.curves.len());

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

                point_a.neighbor = Some(intersection_b.len());
                point_b.neighbor = Some(intersection_a.len());

                intersection_a.push(point_a);
                intersection_b.push(point_b);
            }
        }
    }

    let intersection_in_a = intersection_a.len();
    let intersection_a = create_all_shape(a, intersection_a);

    let intersection_in_b = intersection_b.len();
    let intersection_b = create_all_shape(b, intersection_b);
    (
        GreinerShape::new(intersection_a, intersection_in_a, intersection_in_a),
        GreinerShape::new(intersection_b, intersection_in_b, intersection_in_b),
    )
}

fn create_all_shape(
    shape: &Shape,
    mut intersections: Vec<CoordOfIntersection>,
) -> Vec<CoordOfIntersection> {
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

    result
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

fn merge(ag: &GreinerShape, bg: &GreinerShape, a: &Shape, _b: &Shape) -> Shape {
    let first_intersection = {
        let mut current = &ag.data[ag.start];
        while !current.intersect {
            let next = &ag.data[current.next.unwrap()];
            current = next;
        }
        current
    };

    let mut merged = Shape {
        start: first_intersection.coord_ptr(),
        curves: Vec::new(),
        color: a.color.clone(),
    };

    let mut current = first_intersection;
    let mut current_shape = ag;
    loop {
        if !current.entry {
            //Remove ! to make the algo A AND B
            loop {
                let next = current.next.unwrap();
                current = &current_shape.data[next];
                let cp0 = current.coord_ptr();

                let next = current.next.unwrap();
                current = &current_shape.data[next];
                let cp1 = current.coord_ptr();

                let next = current.next.unwrap();
                current = &current_shape.data[next];
                let p1 = current.coord_ptr();

                merged.curves.push(Curve::new(cp0, cp1, p1));

                if current.intersect {
                    break;
                }
            }
        } else {
            loop {
                let next = current.prev.unwrap();
                current = &current_shape.data[next];
                let cp0 = current.coord_ptr();

                let next = current.prev.unwrap();
                current = &current_shape.data[next];
                let cp1 = current.coord_ptr();

                let next = current.prev.unwrap();
                current = &current_shape.data[next];
                let p1 = current.coord_ptr();

                merged.curves.push(Curve::new(cp0, cp1, p1));

                if current.intersect {
                    break;
                }
            }
        }
        let next = current.neighbor.unwrap();
        let eq = std::ptr::eq(current_shape, ag);
        if eq {
            current_shape = bg;
        } else {
            current_shape = ag;
        }
        current = &current_shape.data[next];

        // first interaction is from ag
        if std::ptr::eq(current, first_intersection)
            || std::ptr::eq(
                current,
                bg.data.get(first_intersection.neighbor.unwrap()).unwrap(),
            )
        {
            break;
        }
    }

    merged
}

#[allow(dead_code)]
pub fn shape_union(a: &Shape, b: &Shape) -> Shape {
    let (mut ag, mut bg) = find_all_intersecion(a, b);

    mark_entry_exit_points(&mut ag, a, &mut bg, b);

    merge(&ag, &bg, a, b)
}

#[cfg(test)]
mod test {
    use common::{types::Coord, Rgba};

    use crate::{
        create_circle,
        shape_boolean::{find_all_intersecion, mark_entry_exit_points, shape_union},
        Vgc,
    };

    #[test]
    fn given_two_circle_when_union_then_valid() {
        let mut vgc = Vgc::new(Rgba::new(255, 255, 255, 255));

        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.2, 0.2);
        create_circle(&mut vgc, Coord::new(0.2, 0.0), 0.2, 0.2);

        let a = vgc.get_shape(0).expect("Shape should exist");
        let b = vgc.get_shape(1).expect("Shape should exist");

        let (mut ag, mut bg) = find_all_intersecion(a, b);

        assert_eq!(ag.len(), 18);
        assert_eq!(bg.len(), 18);

        mark_entry_exit_points(&mut ag, a, &mut bg, b);

        assert_eq!(ag.get(0).entry, false);
        assert_eq!(ag.get(3).entry, true);
        assert_eq!(ag.get(9).entry, false);

        assert_eq!(bg.get(0).entry, false);
        assert_eq!(bg.get(9).entry, true);
        assert_eq!(bg.get(15).entry, false);

        let merged = shape_union(&a, &b);
        assert_eq!(merged.curves.len(), 8);

        let steps = 5;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &Coord::new(x, y);
                assert_eq!(
                    merged.contains(&coord),
                    a.contains(&coord) || b.contains(&coord),
                    "Contains failed at ({}, {})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn given_two_oval_with_no_valid_p_when_union_then_valid() {
        let mut shape1 = vec![
            Coord::new(0.0, 0.3),
            Coord::new(-0.8, 0.3),
            Coord::new(-0.8, -0.3),
            Coord::new(0.0, -0.3),
            Coord::new(0.8, -0.3),
            Coord::new(0.8, 0.3),
            Coord::new(0.0, 0.3),
        ];
        shape1.reverse();

        let shape2 = vec![
            Coord::new(0.3, 0.0),
            Coord::new(0.3, 0.8),
            Coord::new(-0.3, 0.8),
            Coord::new(-0.3, 0.0),
            Coord::new(-0.3, -0.8),
            Coord::new(0.3, -0.8),
            Coord::new(0.3, 0.0),
        ];

        let vgc = crate::generate_from_push(vec![shape1, shape2]);

        let a = vgc.get_shape(0).expect("Shape should exist");
        let b = vgc.get_shape(1).expect("Shape should exist");

        let (mut ag, mut bg) = find_all_intersecion(a, b);

        mark_entry_exit_points(&mut ag, a, &mut bg, b);

        assert_eq!(ag.len(), 18);
        assert_eq!(bg.len(), 18);

        let merged = shape_union(&a, &b);

        assert_eq!(merged.curves.len(), 4);
        println!("Merged curves: {:?}", merged.to_path());

        let steps = 5;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &Coord::new(x, y);
                assert_eq!(
                    merged.contains(&coord),
                    a.contains(&coord) || b.contains(&coord),
                    "Contains failed at ({}, {})",
                    x,
                    y
                );
            }
        }
    }
}
