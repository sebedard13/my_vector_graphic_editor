use super::GreinerShape;
use super::IntersectionType;
use crate::math::curve::cubic_bezier;
use crate::scene::shape::Shape;
use anyhow::Context;
use anyhow::Error;
use common::types::Coord;

pub(super) fn mark_entry_exit_points(
    ag: &mut GreinerShape,
    a: &Shape,
    bg: &mut GreinerShape,
    b: &Shape,
) -> Result<(), anyhow::Error> {
    mark_shape_entries(ag, bg, b)?;
    mark_shape_entries(bg, ag, a)?;
    Ok(())
}

fn mark_shape_entries(
    shape: &mut GreinerShape,
    other_greiner: &GreinerShape,
    other: &Shape,
) -> Result<(), anyhow::Error> {
    let mut run = || -> Result<(), anyhow::Error> {
        let mut status_entry = true;
        let start_index = find_p_not_intersection(shape)?;
        let coord = &shape
            .data
            .get(start_index)
            .context("Index out of bounds")?
            .coord;
        let con = other.contains(coord);
        if con {
            status_entry = false;
        }
        #[cfg(test)]
        println!("Coord {:?} is inside: {}", coord, con);

        set_common_in_out(shape, other_greiner)?;

        run_mark_entry(shape, start_index, status_entry)?;

        #[cfg(test)]
        shape.print_coords_table();

        Ok(())
    };

    run().context("Could not define in out of the shape")
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

fn find_p_not_intersection(shape: &mut GreinerShape) -> Result<usize, anyhow::Error> {
    let mut current_index = shape.start;
    let mut count = 0;
    let mut current_coord = shape
        .data
        .get(current_index)
        .context("Index out of bounds")?;
    while current_coord.intersect != IntersectionType::None {
        (current_index, current_coord) = shape.move_by(current_index, 3, true)?;
        count += 3;
        if count > shape.data.len() {
            return Err(anyhow::anyhow!("No point are not an intersection"));
        }
    }
    Ok(current_index)
}

fn run_mark_entry(
    shape: &mut GreinerShape,
    start_index: usize,
    mut status_entry: bool,
) -> Result<(), Error> {
    let mut current_index = start_index;
    for _ in (0..(shape.data.len() + 2)).step_by(3) {
        let current = shape.move_by_mut(current_index, 3, true)?;
        current_index = current.0;
        let current = current.1;

        if current.intersect.is_intersection() {
            current.entry = status_entry;
            status_entry = !status_entry;
        } else if current.intersect.is_common() {
            current.entry = status_entry;
        }
    }
    Ok(())
}

fn is_overlapping(
    shape: &mut GreinerShape,
    next_index: usize,
    other: &GreinerShape,
    direction: bool,
) -> bool {
    let (p0, cp0, cp1, p1) = shape.next_curve(next_index, direction).unwrap();

    let other_index = shape.data[next_index]
        .neighbor
        .context("Intersection has no neighbor ??")
        .unwrap();
    let other_curves_to_check: [(Coord, Coord, Coord, Coord); 2] = {
        let a = other.next_curve(other_index, true).unwrap();
        let b = other.next_curve(other_index, false).unwrap();
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
