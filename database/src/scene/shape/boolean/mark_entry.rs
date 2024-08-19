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
    mark_shape_entries(ag, bg, b);
    mark_shape_entries(bg, ag, a);
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

    set_common_in_out(shape, other_greiner).unwrap();

    run_mark_entry(shape, start_index, status_entry);

    #[cfg(test)]
    shape.print_coords_table();
}

fn set_common_in_out(
    shape: &mut GreinerShape,
    other_greiner: &GreinerShape,
) -> Result<(), anyhow::Error> {
    for i in 0..shape.intersections_len {
        if shape.data[i].intersect == IntersectionType::UnspecifiedCommonIntersection {
            let is_overlapping_forward = is_overlapping(shape, i, other_greiner, true);
            let is_overlapping_backward = is_overlapping(shape, i, other_greiner, false);
            if is_overlapping_forward && is_overlapping_backward {
                shape.data[i].intersect = IntersectionType::Common;
            } else if is_overlapping_forward {
                shape.data[i].intersect = IntersectionType::IntersectionCommon;
            } else if is_overlapping_backward {
                shape.data[i].intersect = IntersectionType::CommonIntersection;
            } else {
                shape.data[i].intersect = IntersectionType::Intersection;
            }
        }
    }

    Ok(())
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

fn run_mark_entry(shape: &mut GreinerShape, start_index: usize, mut status_entry: bool) {
    let mut current_index = start_index;
    while shape.data[current_index].next.is_some()
        && shape.data[current_index].next.unwrap() != start_index
    {
        let next_index = shape.data[current_index].next.unwrap();
        if shape.data[next_index].intersect.is_intersection() {
            shape.data[next_index].entry = status_entry;
            status_entry = !status_entry;
        } else if shape.data[next_index].intersect.is_common() {
            shape.data[next_index].entry = status_entry;
        }
        current_index = next_index;
    }
}

fn is_overlapping(
    shape: &mut GreinerShape,
    next_index: usize,
    other: &GreinerShape,
    direction: bool,
) -> bool {
    let (p0, cp0, cp1, p1) = shape.next_four_coord(next_index, direction).unwrap();

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
