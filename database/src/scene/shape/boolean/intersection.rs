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
    let (intersections_a, intersections_b) = find_intersecions(a, b);

    if empty_intersection(&intersections_a) && empty_intersection(&intersections_b) {
        if a.contains(&b.path[0].coord) {
            return ShapeIntersection::B;
        } else if b.contains(&a.path[0].coord) {
            return ShapeIntersection::A;
        } else {
            return ShapeIntersection::None;
        }
    }

    let mut ag = create_shape(a, intersections_a);
    let mut bg = create_shape(b, intersections_b);

    mark_entry_exit_points(&mut ag, a, &mut bg, b);

    let merged_shapes = do_intersection(&ag, &bg, a, b);

    ShapeIntersection::New(merged_shapes)
}

fn empty_intersection(intersections: &Vec<CoordOfIntersection>) -> bool {
    for i in 0..intersections.len() {
        if intersections[i].intersect.is_intersection() {
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

                    if current.intersect == IntersectionType::Intersection {
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

                    if current.intersect == IntersectionType::Intersection {
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

    use crate::{scene::shape::Shape, DbCoord};

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
}
