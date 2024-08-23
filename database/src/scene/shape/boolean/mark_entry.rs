use super::CoordOfIntersection;
use super::Direction;
use super::GreinerShape;
use super::IntersectionType;
use crate::math::curve::cubic_bezier;
use crate::math::curve3::curve_realy_intersect;
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
    set_common_in_out(ag, bg)?;
    set_common_in_out(bg, ag)?;

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
        handle_non_intersection(shape, other_greiner)?;

        let mut status_entry = true;
        let start_index = shape.find_first_p_not_intersection()?;
        let coord = &shape
            .data
            .get(start_index)
            .context("Index out of bounds")?
            .coord;
        if other.contains(coord) {
            status_entry = false;
        }
        #[cfg(test)]
        println!("Coord {:?} is inside: {}", coord, status_entry);

        run_mark_entry(shape, start_index, status_entry)?;

        #[cfg(test)]
        shape.print_coords_table();

        Ok(())
    };

    run().context("Could not define in out of the shape")
}

struct ExtractedCurves {
    pub index_c0: usize,
    pub coords_c0: (Coord, Coord, Coord, Coord),
    pub coords_neighbor_c0: (Coord, Coord, Coord, Coord),
    pub index_c1: usize,
    pub coords_c1: (Coord, Coord, Coord, Coord),
    pub coords_neighbor_c1: (Coord, Coord, Coord, Coord),
}

fn handle_non_intersection(shape: &mut GreinerShape, other: &GreinerShape) -> Result<(), Error> {
    let mut extracteds = Vec::<ExtractedCurves>::new();
    let mut intersection: Vec<(usize, &CoordOfIntersection)> = shape
        .data
        .iter()
        .enumerate()
        .take(shape.intersections_len)
        .collect();
    intersection.sort_by(|a, b| a.1.next.unwrap().cmp(&b.1.next.unwrap()));

    for (i, (index_in_shape, _)) in (&intersection).into_iter().enumerate() {
        let index_in_shape = *index_in_shape;
        match shape.data[index_in_shape].intersect {
            IntersectionType::Intersection => {
                let coords_c0 = shape.next_curve(index_in_shape, Direction::Backward)?;
                let coords_c1 = shape.next_curve(index_in_shape, Direction::Forward)?;

                let other_index = shape.data[index_in_shape].neighbor.unwrap();
                assert_eq!(other_index, index_in_shape);

                let coords_neighbor_c0 = other.next_curve(index_in_shape, Direction::Backward)?;
                let coords_neighbor_c1 = other.next_curve(index_in_shape, Direction::Forward)?;
                extracteds.push(ExtractedCurves {
                    index_c0: index_in_shape,
                    coords_c0,
                    coords_neighbor_c0,
                    index_c1: index_in_shape,
                    coords_c1,
                    coords_neighbor_c1,
                });
            }
            IntersectionType::IntersectionCommon => {
                //Maybe not valid ??
                let mut other_index_in_intersection = (i + 1) % intersection.len();
                loop {
                    let other_index = intersection[other_index_in_intersection].0;
                    if shape.data[other_index].intersect == IntersectionType::CommonIntersection {
                        break;
                    }
                    other_index_in_intersection =
                        (other_index_in_intersection + 1) % intersection.len();
                }
                let other_index = intersection[other_index_in_intersection].0;

                let coords_c0 = shape.next_curve(index_in_shape, Direction::Backward)?;
                let coords_c1 = shape.next_curve(other_index, Direction::Forward)?;

                let other_coordf = &other.data[index_in_shape];

                let (coords_neighbor_c0, coords_neighbor_c1) = {
                    if other_coordf.intersect == IntersectionType::IntersectionCommon {
                        (
                            other.next_curve(index_in_shape, Direction::Backward)?,
                            other.next_curve(other_index, Direction::Forward)?,
                        )
                    } else {
                        (
                            other.next_curve(other_index, Direction::Backward)?,
                            other.next_curve(index_in_shape, Direction::Forward)?,
                        )
                    }
                };

                extracteds.push(ExtractedCurves {
                    index_c0: index_in_shape,
                    coords_c0,
                    coords_neighbor_c0,
                    index_c1: other_index,
                    coords_c1,
                    coords_neighbor_c1,
                });
            }
            _ => {}
        }
    }

    for extracted in extracteds.iter() {
        if !curve_realy_intersect(
            &extracted.coords_c0.3,
            &extracted.coords_c0.2,
            &extracted.coords_c0.1,
            &extracted.coords_c0.0,
            &extracted.coords_c1.0,
            &extracted.coords_c1.1,
            &extracted.coords_c1.2,
            &extracted.coords_c1.3,
            &extracted.coords_neighbor_c0.3,
            &extracted.coords_neighbor_c0.2,
            &extracted.coords_neighbor_c0.1,
            &extracted.coords_neighbor_c0.0,
            &extracted.coords_neighbor_c1.0,
            &extracted.coords_neighbor_c1.1,
            &extracted.coords_neighbor_c1.2,
            &extracted.coords_neighbor_c1.3,
        ) {
            shape.data[extracted.index_c0].intersect = IntersectionType::Common;
            shape.data[extracted.index_c1].intersect = IntersectionType::Common;
        }
        else{
            println!("{:?}, {:?}, {:?}, {:?}", extracted.coords_c0, extracted.coords_c1, extracted.coords_neighbor_c0, extracted.coords_neighbor_c1);
        }
    }
    Ok(())
}

fn set_common_in_out(
    shape: &mut GreinerShape,
    other_greiner: &GreinerShape,
) -> Result<(), anyhow::Error> {
    for i in 0..shape.intersections_len {
        if shape.data[i].intersect == IntersectionType::UnspecifiedCommonIntersection {
            let is_overlapping_forward =
                is_overlapping(shape, i, other_greiner, Direction::Forward)?;
            let is_overlapping_backward =
                is_overlapping(shape, i, other_greiner, Direction::Backward)?;
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

fn run_mark_entry(
    shape: &mut GreinerShape,
    start_index: usize,
    mut status_entry: bool,
) -> Result<(), Error> {
    let mut current_index = start_index;
    for _ in (0..(shape.data.len() + 2)).step_by(3) {
        let current = shape.move_by_mut(current_index, 3, Direction::Forward)?;
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
    direction: Direction,
) -> Result<bool, Error> {
    let (p0, cp0, cp1, p1) = shape.next_curve(next_index, direction)?;

    let other_index = shape.data[next_index]
        .neighbor
        .context("Intersection has no neighbor ??")?;
    let other_curves_to_check: [(Coord, Coord, Coord, Coord); 2] = {
        let a = other.next_curve(other_index, Direction::Forward)?;
        let b = other.next_curve(other_index, Direction::Backward)?;
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
            return Ok(true);
        }
    }
    Ok(false)
}
