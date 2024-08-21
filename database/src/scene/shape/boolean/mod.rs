/*
Implementation of boolean operations on shapes.
Union : A OR B
Intersection : A AND B
Difference : A NOT B
*/

mod difference;
mod intersection;
mod union;

mod mark_entry;
use mark_entry::mark_entry_exit_points;

use crate::{
    math::{
        curve::{add_smooth_result, is_line},
        curve2::{intersection, IntersectionResult},
    },
    scene::shape::Shape,
    DbCoord,
};
use anyhow::{Context, Error};
use common::types::Coord;
use std::fmt::Display;

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
/// It contains a double linked list of CoordOfIntersection.
struct GreinerShape {
    pub data: Vec<CoordOfIntersection>,
    pub start: usize,
    pub intersections_len: usize,
}

enum Direction {
    Forward,
    Backward,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum IntersectionType {
    Intersection,
    UnspecifiedCommonIntersection,
    CommonIntersection,
    IntersectionCommon,
    Common,
    None,
}

impl IntersectionType {
    pub fn is_intersection(&self) -> bool {
        match self {
            IntersectionType::Intersection => true,
            IntersectionType::CommonIntersection => true,
            IntersectionType::UnspecifiedCommonIntersection => true,
            _ => false,
        }
    }

    pub fn is_any_intersection(&self) -> bool {
        match self {
            IntersectionType::Intersection => true,
            IntersectionType::CommonIntersection => true,
            IntersectionType::UnspecifiedCommonIntersection => true,
            IntersectionType::IntersectionCommon => true,
            _ => false,
        }
    }

    pub fn is_common(&self) -> bool {
        match self {
            IntersectionType::Common => true,
            IntersectionType::CommonIntersection => true,
            IntersectionType::IntersectionCommon => true,
            _ => false,
        }
    }
}

impl Display for IntersectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntersectionType::Intersection => write!(f, "Inter."),
            IntersectionType::Common => write!(f, "Common"),
            IntersectionType::None => write!(f, "None"),
            IntersectionType::CommonIntersection => write!(f, "C.Int."),
            IntersectionType::IntersectionCommon => write!(f, "Int.C."),
            IntersectionType::UnspecifiedCommonIntersection => write!(f, "Unspec."),
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
    pub intersect: IntersectionType,
    pub coord: Coord,
    pub rel_coord: Option<DbCoord>,
}

impl GreinerShape {
    pub fn new(data: Vec<CoordOfIntersection>, start: usize, intersections_len: usize) -> Self {
        Self {
            data,
            start,
            intersections_len,
        }
    }

    pub fn move_by(
        &self,
        start: usize,
        mut count: usize,
        direction: Direction,
    ) -> Result<(usize, &CoordOfIntersection), Error> {
        let mut current = start;
        while count > 0 {
            let value = self.data.get(current).context("Index out of bound")?;
            match direction {
                Direction::Forward => {
                    current = value.next.context("Invalid No next")?;
                }
                Direction::Backward => {
                    current = value.prev.context("Invalid No prev")?;
                }
            }
            count -= 1;
        }
        Ok((current, &self.data[current]))
    }

    pub fn move_by_mut(
        &mut self,
        start: usize,
        mut count: usize,
        direction: Direction,
    ) -> Result<(usize, &mut CoordOfIntersection), Error> {
        let mut current = start;
        while count > 0 {
            let value = self.data.get(current).context("Index out of bound")?;
            match direction {
                Direction::Forward => {
                    current = value.next.context("Invalid No next")?;
                }
                Direction::Backward => {
                    current = value.prev.context("Invalid No prev")?;
                }
            }
            count -= 1;
        }
        Ok((current, &mut self.data[current]))
    }

    pub fn next_curve(
        &self,
        mut current_index: usize,
        direction: Direction,
    ) -> Result<(Coord, Coord, Coord, Coord), anyhow::Error> {
        match direction {
            Direction::Forward => {
                let p0 = self.data.get(current_index).context("No next")?;
                current_index = p0.next.context("No next")?;
                let cp0 = self.data.get(current_index).context("No next")?;
                current_index = cp0.next.context("No next")?;
                let cp1 = self.data.get(current_index).context("No next")?;
                current_index = cp1.next.context("No next")?;
                let p1 = self.data.get(current_index).context("No next")?;
                Ok((p0.coord, cp0.coord, cp1.coord, p1.coord))
            }
            Direction::Backward => {
                let p0 = self.data.get(current_index).context("No next")?;
                current_index = p0.prev.context("No next")?;
                let cp0 = self.data.get(current_index).context("No next")?;
                current_index = cp0.prev.context("No next")?;
                let cp1 = self.data.get(current_index).context("No next")?;
                current_index = cp1.prev.context("No next")?;
                let p1 = self.data.get(current_index).context("No next")?;
                Ok((p0.coord, cp0.coord, cp1.coord, p1.coord))
            }
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
        println!(
            "{:>7}, {:>7}, {:>7}, {:>7}, {:>7}, {:>7}, {:>7}, {:>7}",
            "Index", "x", "y", "In.sect", "Next", "Prev", "Neigh.", "Entry"
        );
        for (i, c) in self.data.iter().enumerate() {
            let x = format!("{:.4}", c.coord.x());
            let y = format!("{:.4}", c.coord.y());
            let neighbor = format!("{:?}", c.neighbor);
            let int = format!("{}", c.intersect);

            println!(
                "{:>7}, {:>7}, {:>7}, {:>7}, {:>7}, {:>7}, {:>7}, {:>7}",
                i,
                x,
                y,
                int,
                c.next.unwrap_or(usize::MAX),
                c.prev.unwrap_or(usize::MAX),
                neighbor,
                c.entry,
            );
        }
    }
}

impl CoordOfIntersection {
    pub fn from_existing(rel_coord: &DbCoord) -> Self {
        Self {
            curve_index: 0, //we don't need this
            t: 0.0,         //we don't need this
            neighbor: None,
            next: None,
            prev: None,
            entry: false,
            intersect: IntersectionType::None,
            coord: rel_coord.coord,
            rel_coord: Some(*rel_coord),
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
            intersect: IntersectionType::Intersection,
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
            intersect: IntersectionType::None,
            coord: coord,
            rel_coord: None,
        }
    }

    pub fn coord_ptr(&self) -> DbCoord {
        match &self.rel_coord {
            Some(rel_coord) => rel_coord.clone(),
            None => DbCoord::new(self.coord.x(), self.coord.y()),
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
    let mut intersections_a: Vec<CoordOfIntersection> = Vec::with_capacity(a.path.len());
    let mut intersections_b: Vec<CoordOfIntersection> = Vec::with_capacity(b.path.len());

    for (i, a_curve) in a.curves().enumerate() {
        for (j, b_curve) in b.curves().enumerate() {
            let intersection_result = intersection(
                &a_curve.p0.coord,
                &a_curve.cp0.coord,
                &a_curve.cp1.coord,
                &a_curve.p1.coord,
                &b_curve.p0.coord,
                &b_curve.cp0.coord,
                &b_curve.cp1.coord,
                &b_curve.p1.coord,
            );

            match intersection_result {
                IntersectionResult::None => {}
                IntersectionResult::Pts(intersection_points) => {
                    for point in intersection_points {
                        //continue if point is already in the list
                        if intersections_a.iter().any(|x| &x.coord == &point.coord) {
                            continue;
                        }

                        let mut point_a =
                            CoordOfIntersection::from_intersection(point.coord, point.t1, i);

                        let mut point_b =
                            CoordOfIntersection::from_intersection(point.coord, point.t2, j);
                        if point_a.t == 1.0 {
                            point_a.t = 0.0;
                            point_a.curve_index = (point_a.curve_index + 1) % a.curves_len();
                            point_a.rel_coord = Some(*a_curve.p1);
                            point_b.rel_coord = Some(*a_curve.p1);
                        } else if point_a.t == 0.0 {
                            point_a.rel_coord = Some(*a_curve.p0);
                            point_b.rel_coord = Some(*a_curve.p0);
                        }
                        if point_b.t == 1.0 {
                            point_b.t = 0.0;
                            point_b.curve_index = (point_b.curve_index + 1) % b.curves_len();
                            point_b.rel_coord = Some(*b_curve.p1);
                            point_a.rel_coord = Some(*b_curve.p1);
                        } else if point_b.t == 0.0 {
                            point_b.rel_coord = Some(*b_curve.p0);
                            point_a.rel_coord = Some(*b_curve.p0);
                        }

                        if point_a.t == 0.0 && point_b.t == 0.0 {
                            point_a.intersect = IntersectionType::UnspecifiedCommonIntersection;
                            point_b.intersect = IntersectionType::UnspecifiedCommonIntersection;
                        } else if point_a.t == 0.0 || point_b.t == 0.0 {
                            //Will flip later if neccesary
                            point_a.intersect = IntersectionType::UnspecifiedCommonIntersection;
                            point_b.intersect = IntersectionType::UnspecifiedCommonIntersection;
                        }

                        point_a.neighbor = Some(intersections_b.len());
                        point_b.neighbor = Some(intersections_a.len());

                        intersections_a.push(point_a);
                        intersections_b.push(point_b);
                    }
                }
                _ => {}
            }
        }
    }

    (intersections_a, intersections_b)
}

fn create_shape(shape: &Shape, mut intersections: Vec<CoordOfIntersection>) -> GreinerShape {
    let mut result = Vec::with_capacity(shape.curves_len() * 3 + intersections.len() * 3);
    result.append(&mut intersections.clone());
    result.push(CoordOfIntersection::from_existing(&shape.path[0]));

    let mut current_index = result.len() - 1;
    let mut start_a = result.len() - 1;

    intersections.sort();
    let mut iter = intersections.iter();
    let mut current_intersection = iter.next();

    for (curve_index, curve) in shape.curves().enumerate() {
        let (a_p0, a_cp0, a_cp1, a_p1) = (curve.p0, curve.cp0, curve.cp1, curve.p1);
        let mut current_p0 = (a_p0.coord, Some(a_p0));
        let mut current_cp0 = (a_cp0.coord, Some(a_cp0));
        let mut current_cp1 = (a_cp1.coord, Some(a_cp1));
        let current_p1 = (a_p1.coord, a_p1);

        let mut last_t = 0.0;
        while current_intersection.is_some()
            && current_intersection.unwrap().curve_index == curve_index
        {
            let intersection = current_intersection.unwrap();

            let t = (intersection.t - last_t) / (1.0 - last_t);

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

    let shape = GreinerShape::new(result, start_a, intersections.len());
    shape
}

fn compress_coord_ptr(list: &mut Vec<CoordOfIntersection>, start_a: usize) {
    let mut current_index = start_a;

    let mut i_p0 = start_a;
    for _ in (0..(list.len() + 2)).step_by(3) {
        current_index = list[current_index].next.unwrap();
        let i_cp0 = current_index;

        current_index = list[current_index].next.unwrap();
        let i_cp1 = current_index;

        current_index = list[current_index].next.unwrap();
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
}

/// Basic tests for the boolean operations.
/// The tests are based on basic shapes and the expected result can be seen easily.
#[cfg(test)]
mod basic_test;

#[cfg(test)]
mod test {
    use common::{
        pures::Affine,
        types::{Coord, Length2d},
    };

    use super::{create_shape, mark_entry::mark_entry_exit_points};
    use crate::{scene::shape::Shape, DbCoord};

    use super::find_intersecions;

    #[test]
    fn given_bug_diff_when_difference() {
        // A: M 1 -1 C 1 -1 1 1 1 1 C 1 1 0 0 0 0 C 0 0 -1 -1 -1 -1 C -1 -1 1 -1 1 -1 Z wasm_client_bg.js:1982:12
        //B:  C -0.5066913 -0.54993117 -0.50866926 -0.53057325 -0.5123266 -0.5123266 Z

        let coords_a = vec![
            DbCoord::new(1.0, -1.0),
            DbCoord::new(1.0, -1.0),
            DbCoord::new(1.0, 1.0),
            DbCoord::new(1.0, 1.0),
            DbCoord::new(1.0, 1.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(1.0, -1.0),
            DbCoord::new(1.0, -1.0),
        ];

        let coords_b = vec![
            DbCoord::new(-0.5123266, -0.5123266),
            DbCoord::new(-0.25828606, -0.25828606),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(1.0, 1.0),
            DbCoord::new(1.0, 1.0),
            DbCoord::new(1.0, 1.0),
            DbCoord::new(-1.0, 1.0),
            DbCoord::new(-1.0, 1.0),
            DbCoord::new(-1.0, 1.0),
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(-0.880739, -0.880739),
            DbCoord::new(-0.7245886, -0.7245886),
            DbCoord::new(-0.7016079, -0.75289893),
            DbCoord::new(-0.67217433, -0.7698959),
            DbCoord::new(-0.6400001, -0.77001095),
            DbCoord::new(-0.56620985, -0.76974714),
            DbCoord::new(-0.50683534, -0.6806853),
            DbCoord::new(-0.5066594, -0.56999993),
            DbCoord::new(-0.5066913, -0.54993117),
            DbCoord::new(-0.50866926, -0.53057325),
            DbCoord::new(-0.5123266, -0.5123266),
        ];

        let a = Shape::new_from_path(coords_a, Affine::identity());
        let b = Shape::new_from_path(coords_b, Affine::identity());

        let intersections = find_intersecions(&a, &b);
        let mut ag = create_shape(&a, intersections.0);
        let mut bg = create_shape(&b, intersections.1);

        mark_entry_exit_points(&mut ag, &a, &mut bg, &b).unwrap();

        let mut common_count = 0;
        let mut inteersection_count = 0;
        for i in 0..ag.intersections_len {
            if ag.data[i].intersect.is_common() {
                common_count += 1;
            }
            if ag.data[i].intersect.is_any_intersection() {
                inteersection_count += 1;
                assert!(
                    ag.data[i].t == 0.654658318
                        || ag.data[i].t == 0.508217812
                        || ag.data[i].t == 0.0,
                    "t should be 0.654658138 or 0.508218467, but was {}",
                    ag.data[i].t
                );
            }
        }
        assert_eq!(inteersection_count, 4);
        assert_eq!(common_count, 5);
    }

    #[test]
    fn given_two_circle_when_union_then_new() {
        let a = &Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.2, 0.2));
        let b = &Shape::new_circle(Coord::new(0.2, 0.0), Length2d::new(0.2, 0.2));

        let (i_a, i_b) = find_intersecions(a, b);

        assert_eq!(i_a.len(), 2);
        assert_eq!(i_b.len(), 2);

        let mut ag = create_shape(a, i_a);
        let mut bg = create_shape(b, i_b);

        assert_eq!(ag.len(), 18);
        assert_eq!(bg.len(), 18);

        mark_entry_exit_points(&mut ag, a, &mut bg, b).unwrap();

        assert_eq!(ag.get(0).entry, false);
        assert_eq!(ag.get(3).entry, true);
        assert_eq!(ag.get(9).entry, false);

        assert_eq!(bg.get(0).entry, false);
        assert_eq!(bg.get(9).entry, true);
        assert_eq!(bg.get(15).entry, false);
    }

    #[test]
    fn given_two_oval_with_no_valid_p_when_union_then_new() {
        let mut shape1 = vec![
            DbCoord::new(0.0, 0.3),
            DbCoord::new(-0.8, 0.3),
            DbCoord::new(-0.8, -0.3),
            DbCoord::new(0.0, -0.3),
            DbCoord::new(0.8, -0.3),
            DbCoord::new(0.8, 0.3),
            DbCoord::new(0.0, 0.3),
        ];
        shape1.reverse();

        let shape2 = vec![
            DbCoord::new(0.3, 0.0),
            DbCoord::new(0.3, 0.8),
            DbCoord::new(-0.3, 0.8),
            DbCoord::new(-0.3, 0.0),
            DbCoord::new(-0.3, -0.8),
            DbCoord::new(0.3, -0.8),
            DbCoord::new(0.3, 0.0),
        ];

        let a = &Shape::new_from_path(shape1, Affine::identity());
        let b = &Shape::new_from_path(shape2, Affine::identity());

        let (i_a, i_b) = find_intersecions(a, b);

        assert_eq!(i_a.len(), 4);
        assert_eq!(i_b.len(), 4);

        let mut ag = create_shape(a, i_a);
        let mut bg = create_shape(b, i_b);

        mark_entry_exit_points(&mut ag, a, &mut bg, b).unwrap();

        assert_eq!(ag.len(), 18);
        assert_eq!(bg.len(), 18);
    }
}
