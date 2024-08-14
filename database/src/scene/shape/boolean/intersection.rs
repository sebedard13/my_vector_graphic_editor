use anyhow::Error;

use super::{
    create_shape, find_intersecions, mark_entry_exit_points, CoordOfIntersection, GreinerShape,
    IntersectionType,
};
use crate::scene::shape::Shape;

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
    if empty_intersection(&intersections_a) && empty_intersection(&intersections_b) {
        //may have common intersection

        let mut common_curve_check = vec![false; a.curves_len()];
        for i in 0..intersections_a.len() {
            if intersections_a[i].intersect == IntersectionType::CommonIntersection {
                common_curve_check[intersections_a[i].curve_index] = true;
            }
        }

        if let Some(index_curve_not_common) = find_index_false(&common_curve_check) {
            if b.contains(&a.curve_select(index_curve_not_common).unwrap().p0.coord) {
                return Ok(ShapeIntersection::A);
            } else if a.contains(&b.path[0].coord) {
                return Ok(ShapeIntersection::B);
            } else {
                return Ok(ShapeIntersection::None);
            }
        }

        return Ok(ShapeIntersection::A);
    }
    let mut ag = create_shape(a, intersections_a);
    let mut bg = create_shape(b, intersections_b);
    mark_entry_exit_points(&mut ag, a, &mut bg, b);
    let merged_shapes = do_intersection(&ag, &bg, a, b);
    return Ok(ShapeIntersection::New(merged_shapes));
}

fn empty_intersection(intersections: &Vec<CoordOfIntersection>) -> bool {
    if intersections.len() == 0 {
        return true;
    }

    for i in 0..intersections.len() {
        if intersections[i].intersect == IntersectionType::Intersection {
            return false;
        }
    }
    true
}

fn find_index_false(v: &Vec<bool>) -> Option<usize> {
    for (i, b) in v.iter().enumerate() {
        if !b {
            return Some(i);
        }
    }
    None
}

fn do_intersection(ag: &GreinerShape, bg: &GreinerShape, a: &Shape, _b: &Shape) -> Vec<Shape> {
    let mut intersections_done = vec![false; ag.intersections_len];
    let mut shapes = Vec::new();

    let max_visit_count = (ag.len() + bg.len()) * 2;
    let mut visit_count = 0;

    while let Some(i) = find_index_false(&intersections_done) {
        let first_intersection = &ag.data[i];
        intersections_done[i] = true;

        let mut merged = Shape {
            id: a.id,
            path: vec![first_intersection.coord_ptr()],
            color: a.color.clone(),
        };

        let mut current = first_intersection;
        let mut current_shape = ag;
        loop {
            if current.entry {
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

                    if current.intersect.is_intersection() {
                        intersections_done[next] = true;
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

                    if current.intersect.is_intersection() {
                        intersections_done[next] = true;
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
    use super::{shape_intersection, ShapeIntersection};
    use common::{
        pures::{Affine, Vec2},
        types::{Coord, Length2d},
    };

    use crate::{
        scene::shape::{
            boolean::{difference::shape_difference, find_intersecions, ShapeDifference},
            Shape,
        },
        DbCoord,
    };

    #[test]
    fn given_two_circle_when_intersection_then_new_1() {
        let a = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.2, 0.2));
        let b = Shape::new_circle(Coord::new(0.2, 0.0), Length2d::new(0.2, 0.2));

        let merged = shape_intersection(&a, &b);
        let shapes = match merged {
            ShapeIntersection::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].curves_len(), 4);

        let steps = 9;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &DbCoord::new(x, y);
                assert_eq!(
                    shapes[0].contains(&coord.coord),
                    a.contains(&coord.coord) && b.contains(&coord.coord),
                    "Contains failed at ({}, {})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn given_two_oval_with_no_valid_p_when_intersection_then_new_1() {
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

        let a = Shape::new_from_path(shape1, Affine::identity());
        let b = Shape::new_from_path(shape2, Affine::identity());

        let merged = shape_intersection(&a, &b);

        let merged = match merged {
            ShapeIntersection::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].curves_len(), 8);

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
    fn given_two_circle_when_intersection_then_b() {
        let a = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.2, 0.2));
        let b = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.1, 0.1));

        let merged = shape_intersection(&a, &b);

        assert!(
            matches!(merged, ShapeIntersection::B),
            "Should be ShapeIntersection::B"
        );
    }

    #[test]
    fn given_two_circle_when_intersection_then_none() {
        let a = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.2, 0.2));
        let b = Shape::new_circle(Coord::new(0.3, 0.3), Length2d::new(0.1, 0.1));

        let merged = shape_intersection(&a, &b);

        assert!(
            matches!(merged, ShapeIntersection::None),
            "Should be ShapeUnion::None"
        );
    }

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
                .translate(Vec2::new(0.0, -0.07)),
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

        match shape.intersection(&max_view) {
            ShapeIntersection::A => (),
            _ => panic!("Should be ShapeIntersection::A"),
        }
    }

    #[test]
    fn given_shapes_with_common_side_and_intersection_when_intersection_then_new() {
        let max_view = Shape::quick_from_string(
            "M 0 0 C 0 0
            559 0 559 0 C 559 0 
            559 383 559 383 C 559 383 
            0 383 0 383 C 0 383 
            0 0 0 0 Z",
        );

        let shape = Shape::quick_from_string(
            "M 540 0 C 540 0 
            540 45 540 45 C 540 45 
            585 45 585 45 C 585 45
            585 0 585 0 C 585 0
            540 0 540 0 Z",
        );

        match shape.intersection(&max_view) {
            ShapeIntersection::New(new_shape) => {
                assert_eq!(new_shape.len(), 1);
                assert_eq!(new_shape[0].curves_len(), 4);
            }
            _ => panic!("Should be ShapeIntersection::New"),
        }
    }

    #[test]
    fn given_squares_2i_1ci_when_intersection_then_new() {
        let a = Shape::quick_from_string("M 0 360 C 0 360 0 405 0 405 C 0 405 45 405 45 405 C 45 405 45 360 45 360 C 45 360 0 360 0 360 Z");
        //max_view
        let b = Shape::quick_from_string(
            "M 0 0 C 0 0
            559 0 559 0 C 559 0 
            559 383 559 383 C 559 383 
            0 383 0 383 C 0 383 
            0 0 0 0 Z",
        );

        let intersections = super::find_intersecions(&a, &b);

        let mut ag = super::create_shape(&a, intersections.0);
        let mut bg = super::create_shape(&b, intersections.1);

        super::mark_entry_exit_points(&mut ag, &a, &mut bg, &b);
        ag.print_coords_table();
        bg.print_coords_table();

        let merged = shape_difference(&a, &b);

        match merged {
            ShapeDifference::New(merged) => {
                assert_eq!(merged.len(), 1);
                println!("{:?}", merged[0].path());
            }
            _ => panic!("Should be a new shape"),
        };
    }
}
