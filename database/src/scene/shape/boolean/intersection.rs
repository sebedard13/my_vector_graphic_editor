use std::ptr;

use super::{
    create_shape, find_intersecions, mark_entry_exit_points, GreinerShape, IntersectionType,
};
use crate::scene::shape::Shape;
use anyhow::Error;

#[derive(Debug, Clone)]
pub enum ShapeIntersection {
    /// B fully contains A so the result is A
    A,
    /// A fully contains B so the result is B
    B,
    /// A and B do not fully contain each other
    /// New shapes are created.
    /// Multiple shapes can be created exemple shapes like a C and Ↄ. The tips could intersect each other resulting in 2 shapes.
    New(Vec<Shape>),
    /// A and B do not intersect each other
    None,
}

#[allow(dead_code)]
pub fn shape_intersection(a: &Shape, b: &Shape) -> ShapeIntersection {
    match try_shape_intersection(a, b) {
        Ok(result) => result,
        Err(e) => {
            log::error!(
                "Error while trying to intersect a {:?} and b {:?} : {:?}",
                a.path(),
                b.path(),
                e
            );
            ShapeIntersection::None
        }
    }
}

fn try_shape_intersection(a: &Shape, b: &Shape) -> Result<ShapeIntersection, Error> {
    let (intersections_a, intersections_b) = find_intersecions(a, b);
    if intersections_a.is_empty() && intersections_b.is_empty() {
        if b.contains(&a.path[0].coord) {
            return Ok(ShapeIntersection::A);
        } else if a.contains(&b.path[0].coord) {
            return Ok(ShapeIntersection::B);
        } else {
            return Ok(ShapeIntersection::None);
        }
    }

    let mut ag = create_shape(a, intersections_a);
    let mut bg = create_shape(b, intersections_b);
    mark_entry_exit_points(&mut ag, a, &mut bg, b)?;

    if let Some(result) = handle_touching_shape(&ag, &bg)? {
        return Ok(result);
    }

    let merged_shapes = do_intersection(&ag, &bg, a, b);
    Ok(ShapeIntersection::New(merged_shapes))
}

fn handle_touching_shape(
    ag: &GreinerShape,
    bg: &GreinerShape,
) -> Result<Option<ShapeIntersection>, Error> {
    let mut count_intersections = 0;
    for i in 0..ag.intersections_len {
        let current = &ag.data[i];
        if current.intersect.is_intersection() {
            count_intersections += 1;
        }
    }

    if count_intersections % 2 == 1 {
        return Err(anyhow::anyhow!(
            "Odd number of intersections that is illogical"
        ));
    }

    if count_intersections == 0
        && ag.intersections_len * 3 == ag.len()
        && bg.intersections_len * 3 == bg.len()
    {
        return Err(anyhow::anyhow!(
            "Only common intersections and free points. Is it the same shape?"
        ));

        //Yeah and it works for intersection
    }

    if count_intersections == 0 {
        if ag.data[0].entry && bg.data[0].entry {
            return Ok(Some(ShapeIntersection::None));
        } else if ag.data[0].entry && !bg.data[0].entry {
            return Ok(Some(ShapeIntersection::B));
        } else if !ag.data[0].entry && bg.data[0].entry {
            return Ok(Some(ShapeIntersection::A));
        } else {
            unreachable!("If both are not entry, then they are always on the same side");
        }
    }

    Ok(None)
}

fn do_intersection(ag: &GreinerShape, bg: &GreinerShape, a: &Shape, _b: &Shape) -> Vec<Shape> {
    let mut intersections_done = vec![false; ag.intersections_len];
    let mut shapes = Vec::new();

    for (i, intersection_done) in intersections_done
        .iter_mut()
        .enumerate()
        .take(ag.intersections_len)
    {
        let current = &ag.data[i];
        if !(current.intersect == IntersectionType::Intersection
            || (current.intersect == IntersectionType::IntersectionCommon && !current.entry)
            || (current.intersect == IntersectionType::CommonIntersection && current.entry))
        {
            *intersection_done = true;
        }
    }

    let max_visit_count = (ag.len() + bg.len()) * 2;
    let mut visit_count = 0;

    while let Some(i) = intersections_done
        .iter()
        .position(|&is_visited| !is_visited)
    {
        let first_intersection = &ag.data[i];
        intersections_done[i] = true;

        let mut merged = a.clone();
        merged.path = vec![first_intersection.coord_ptr()];

        let mut current = first_intersection;
        let mut current_shape = ag;
        loop {
            if ptr::eq(current_shape, bg) && current.entry {
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

                    merged.path.append(&mut vec![cp0, cp1, p1]);

                    if next < current_shape.intersections_len {
                        intersections_done[next] = true;
                    }

                    if current.intersect == IntersectionType::Intersection
                        || (current.intersect == IntersectionType::IntersectionCommon
                            && current.entry)
                        || (current.intersect == IntersectionType::CommonIntersection
                            && !current.entry)
                    {
                        break;
                    }
                }
            } else if ptr::eq(current_shape, bg) {
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

                    merged.path.append(&mut vec![cp0, cp1, p1]);

                    if next < current_shape.intersections_len {
                        intersections_done[next] = true;
                    }

                    if current.intersect == IntersectionType::Intersection
                        || (current.intersect == IntersectionType::IntersectionCommon
                            && current.entry)
                        || (current.intersect == IntersectionType::CommonIntersection
                            && !current.entry)
                    {
                        break;
                    }
                }
            } else if current.entry {
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

                    merged.path.append(&mut vec![cp0, cp1, p1]);

                    if next < current_shape.intersections_len {
                        intersections_done[next] = true;
                    }

                    if current.intersect == IntersectionType::Intersection
                        || (current.intersect == IntersectionType::IntersectionCommon
                            && !current.entry)
                        || (current.intersect == IntersectionType::CommonIntersection
                            && current.entry)
                    {
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

                    merged.path.append(&mut vec![cp0, cp1, p1]);

                    if next < current_shape.intersections_len {
                        intersections_done[next] = true;
                    }

                    if current.intersect == IntersectionType::Intersection
                        || (current.intersect == IntersectionType::IntersectionCommon
                            && !current.entry)
                        || (current.intersect == IntersectionType::CommonIntersection
                            && current.entry)
                    {
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
            let bg_first_intersection = bg.data.get(first_intersection.neighbor.unwrap()).unwrap();
            if std::ptr::eq(current, first_intersection)
                || std::ptr::eq(current, bg_first_intersection)
            {
                break;
            }

            visit_count += 3;
            if visit_count > max_visit_count {
                panic!("Infinite loop detected");
            }
        }
        shapes.push(merged);
    }

    shapes
}

#[cfg(test)]
mod test {
    use super::{shape_intersection, try_shape_intersection, ShapeIntersection};
    use common::{pures::Affine, types::Coord};

    use crate::{
        scene::shape::{
            boolean::{
                basic_test::{print_svg_scale, verify_intersection},
                find_intersecions,
            },
            Shape,
        },
        DbCoord,
    };

    #[test]
    fn given_two_c_shape_when_intersection_then_new_2() {
        let c_shape_coords = vec![
            DbCoord::new(-0.375, -0.03),
            DbCoord::new(-0.39, -0.41),
            DbCoord::new(0.36, -0.68),
            DbCoord::new(0.221, -0.358),
            DbCoord::new(0.04, -0.08),
            DbCoord::new(-0.25, -0.12),
            DbCoord::new(0.20, 0.33),
            DbCoord::new(0.41, 0.58),
            DbCoord::new(-0.375, 0.22),
            DbCoord::new(-0.375, -0.03),
        ];

        let a = Shape::new_from_path(c_shape_coords.clone(), Affine::identity());
        let b = Shape::new_from_path(
            c_shape_coords,
            Affine::identity()
                .reflect_origin()
                .translate(Coord::new(0.0, -0.07)),
        );

        let merged = shape_intersection(&a, &b);

        let merged = match merged {
            ShapeIntersection::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].curves_len(), 4);
        assert_eq!(merged[1].curves_len(), 4);

        let steps = 7;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &DbCoord::new(x, y);
                assert_eq!(
                    merged[0].contains(&coord.coord) || merged[1].contains(&coord.coord),
                    a.contains(&coord.coord) && b.contains(&coord.coord),
                    "Contains failed at ({}, {})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn given_shape_square_when_intersection_then_new() {
        let a = Shape::quick_from_string("M -90 -90 C -90 -90 -90 -45 -90 -45 C -90 -45 -45 -45 -45 -45 C -45 -45 -45 -90 -45 -90 C -45 -90 -90 -90 -90 -90 Z");
        let b = Shape::quick_from_string("M -95.5 -58.5 C -95.5 -58.5 -95.5 441.5 -95.5 441.5 C -95.5 441.5 654.5 441.5 654.5 441.5 C 654.5 441.5 654.5 -58.5 654.5 -58.5 C 654.5 -58.5 -95.5 -58.5 -95.5 -58.5 Z");

        let (inters_a, _) = find_intersecions(&a, &b);

        assert_eq!(inters_a.len(), 2);

        let merged = shape_intersection(&a, &b);
        let merged = match merged {
            ShapeIntersection::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].curves_len(), 4);

        let steps = 7;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &DbCoord::new(x, y);
                assert_eq!(
                    merged[0].contains(&coord.coord),
                    a.contains(&coord.coord) && b.contains(&coord.coord),
                    "Contains failed at ({}, {})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn given_shapes_with_common_side_then_intersection() {
        let max_view = Shape::quick_from_string(
            "M 0 0 C 0 0
            559 0 559 0 C 559 0 
            559 383 559 383 C 559 383 
            0 383 0 383 C 0 383 
            0 0 0 0 Z",
        );

        let shape = Shape::quick_from_string(
            "M 90 0 C 90 0 
            90 45 90 45 C 90 45 
            135 45 135 45 C 135 45
            135 0 135 0 C 135 0
            90 0 90 0 Z",
        );

        print_svg_scale(&shape, &max_view, 1.0);

        match shape.intersection(&max_view) {
            ShapeIntersection::A => (),
            _ => panic!("Should be ShapeIntersection::A"),
        }
    }

    #[test]
    fn given_shapes_with_common_side_and_intersection_when_intersection_then_new() {
        let a = Shape::quick_from_string(
            "M 0 0 C 0 0
            559 0 559 0 C 559 0 
            559 383 559 383 C 559 383 
            0 383 0 383 C 0 383 
            0 0 0 0 Z",
        );

        let b = Shape::quick_from_string(
            "M 540 0 C 540 0 
            540 45 540 45 C 540 45 
            585 45 585 45 C 585 45
            585 0 585 0 C 585 0
            540 0 540 0 Z",
        );

        match b.intersection(&a) {
            ShapeIntersection::New(new_shape) => {
                assert_eq!(new_shape.len(), 1);
                assert_eq!(new_shape[0].curves_len(), 4);
                assert_eq!(new_shape[0].path(), "M 540 0 C 540 0 540 45 540 45 C 540 45 559 45 559 45 C 559 45 559 0 559 0 C 559 0 540 0 540 0 Z");
            }
            _ => panic!("Should be ShapeIntersection::New"),
        }
    }

    #[test]
    fn bug_intersection_trans() {
        //"M 0 0 C 0 0 0 20 0 20 C 0 20 30 20 30 20 C 30 20 30 0 30 0 C 30 0 0 0 0 0 Z" and b "M 0 0 C 0 0 30 0 30 0 C 30 0 30 30 30 30 C 30 30 0 30 0 30 C 0 30 0 0 0 0 Z"
        let a = Shape::quick_from_string(
            "M 0 0 C 0 0 0 20 0 20 C 0 20 30 20 30 20 C 30 20 30 0 30 0 C 30 0 0 0 0 0 Z",
        );
        let b = Shape::quick_from_string(
            "M 0 0 C 0 0 30 0 30 0 C 30 0 30 30 30 30 C 30 30 0 30 0 30 C 0 30 0 0 0 0 Z",
        );
        print_svg_scale(&a, &b, 1.0);

        let merged = try_shape_intersection(&a, &b).unwrap();

        match merged {
            ShapeIntersection::A => {}
            _ => panic!("Should be A shape"),
        };

        verify_intersection(merged, a, b);
    }

    #[test]
    fn bug2_intersection_trans() {
        //a "M 0 630 C 0 630 0 675 0 675 C 0 675 45 675 45 675 C 45 675 45 630 45 630 C 45 630 0 630 0 630 Z" and b "M 0 672.5 C 0 672.5 724.5002 672.5 724.5002 672.5 C 724.5002 672.5 724.5002 172.5 724.5002 172.5 C 724.5002 172.5 -0.000011444092 172.5 -0.000011444092 172.5 C -0.000011444092 172.5 0 672.5 0 672.5 Z"
        let a = Shape::quick_from_string("M 0 630 C 0 630 0 675 0 675 C 0 675 45 675 45 675 C 45 675 45 630 45 630 C 45 630 0 630 0 630 Z");
        let b = Shape::quick_from_string("M 0 672.5 C 0 672.5 724.5002 672.5 724.5002 672.5 C 724.5002 672.5 724.5002 172.5 724.5002 172.5 C 724.5002 172.5 -0.000011444092 172.5 -0.000011444092 172.5 C -0.000011444092 172.5 0 672.5 0 672.5 Z");

        print_svg_scale(&a, &b, 1.0);

        let merged = try_shape_intersection(&a, &b).unwrap();

        match merged {
            ShapeIntersection::New(_) => {}
            _ => panic!("Should be a new shape"),
        };

        verify_intersection(merged, a, b);
    }
}
