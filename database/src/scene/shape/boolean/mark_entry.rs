use super::GreinerShape;
use super::IntersectionType;
use crate::math::curve::cubic_bezier;
use crate::scene::shape::Shape;
use anyhow::Context;
use common::types::Coord;

pub(super) fn mark_entry_exit_points(
    ag: &mut GreinerShape,
    a: &Shape,
    bg: &mut GreinerShape,
    b: &Shape,
) {
    or_common_intersection(ag, bg);

    mark_shape_entries(ag, bg, b);
    mark_shape_entries(bg, ag, a);
}

fn or_common_intersection(a: &mut GreinerShape, b: &mut GreinerShape) {
    for i in 0..a.intersections_len {
        if a.data[i].intersect == IntersectionType::CommonIntersection {
            b.data[i].intersect = IntersectionType::CommonIntersection;
        } else if b.data[i].intersect == IntersectionType::CommonIntersection {
            a.data[i].intersect = IntersectionType::CommonIntersection;
        }
    }
}

fn mark_shape_entries(shape: &mut GreinerShape, other_greiner: &GreinerShape, other: &Shape) {
    let mut status_entry = true;
    let start_index = find_p_not_intersection(shape);
    let coord = &shape.data[start_index].coord;
    let con = other.contains(coord);
    if con {
        status_entry = false;
    }
    #[cfg(test)]
    println!("Coord {:?} is inside: {}", coord, con);

    run_mark_entry(shape, &other_greiner, start_index, status_entry);

    #[cfg(test)]
    shape.print_coords_table();
}

fn find_p_not_intersection(shape: &mut GreinerShape) -> usize {
    let mut current_index = shape.start;
    let mut count = 0;
    while shape.data[current_index].intersect != IntersectionType::None {
        current_index = shape.data[current_index].next.unwrap();
        current_index = shape.data[current_index].next.unwrap();
        current_index = shape.data[current_index].next.unwrap();
        count += 3;
        if count > shape.data.len() {
            panic!("Infinite loop");
        }
    }
    current_index
}

fn run_mark_entry(
    shape: &mut GreinerShape,
    other: &GreinerShape,
    start_index: usize,
    mut status_entry: bool,
) {
    let mut current_index = start_index;
    while shape.data[current_index].next.is_some()
        && shape.data[current_index].next.unwrap() != start_index
    {
        let next_index = shape.data[current_index].next.unwrap();
        if shape.data[next_index].intersect.is_intersection() {
            let is_overlapping = shape.data[next_index].intersect.is_common()
                && is_overlapping(shape, next_index, other);
            if is_overlapping {
                shape.data[next_index].intersect = IntersectionType::Common;
                shape.data[next_index].entry = status_entry;
                current_index = next_index;
                continue;
            }

            shape.data[next_index].entry = status_entry;
            status_entry = !status_entry;
        }
        current_index = next_index;
    }
}

fn is_overlapping(shape: &mut GreinerShape, next_index: usize, other: &GreinerShape) -> bool {
    let (p0, cp0, cp1, p1) = shape.next_four_coord(next_index, true).unwrap();

    let other_index = shape.data[next_index]
        .neighbor
        .context("Intersection has no neighbor ??")
        .unwrap();
    let other_curves_to_check: [(Coord, Coord, Coord, Coord); 2] = {
        let a = other.next_four_coord(other_index, true).unwrap();
        let b = other.next_four_coord(other_index, false).unwrap();
        [a, b]
    };

    let coord = cubic_bezier(0.1, &p0, &cp0, &cp1, &p1);
    for other_curve in other_curves_to_check.iter() {
        let other_coord = cubic_bezier(
            0.1,
            &other_curve.0,
            &other_curve.1,
            &other_curve.2,
            &other_curve.3,
        );

        if coord == other_coord {
            return true;
        }
    }
    false
}

// fn update_start_to_intersection(shape: &mut GreinerShape) {
//     let mut current_index = shape.start;
//     while !shape.data[current_index].intersect.is_intersection() {
//         current_index = shape.data[current_index].next.unwrap();
//     }
//     shape.start = current_index;
// }
