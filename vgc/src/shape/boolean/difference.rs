use std::ptr;

use super::{create_shape, find_intersecions, mark_entry_exit_points, GreinerShape};
use crate::{curve::Curve, shape::Shape};

pub enum ShapeDifference {
    /// No change to A
    A,
    /// B fully erease A
    EraseA,

    /// A and B do not fully contain each other
    /// New shapes are created.
    /// Multiple shapes can be created exemple shapes like a l and o. The o could be in the middle of the l. The result would be 2 shapes.
    New(Vec<Shape>),

    /// A has a hole the shaoe of B in it.
    AWithBHole,
}

#[allow(dead_code)]
pub fn shape_difference(a: &Shape, b: &Shape) -> ShapeDifference {
    let (intersections_a, intersections_b) = find_intersecions(a, b);

    if intersections_a.is_empty() && intersections_b.is_empty() {
        if a.contains(&b.start.borrow()) {
            return ShapeDifference::AWithBHole;
        } else if b.contains(&a.start.borrow()) {
            return ShapeDifference::EraseA;
        } else {
            return ShapeDifference::A;
        }
    }

    let mut ag = create_shape(a, intersections_a);
    let mut bg = create_shape(b, intersections_b);

    mark_entry_exit_points(&mut ag, a, &mut bg, b);

    let merged_shapes = do_difference(&ag, &bg, a, b);

    ShapeDifference::New(merged_shapes)
}

fn find_index_false(v: &Vec<bool>) -> Option<usize> {
    for (i, b) in v.iter().enumerate() {
        if !b {
            return Some(i);
        }
    }
    None
}

fn do_difference(ag: &GreinerShape, bg: &GreinerShape, a: &Shape, _b: &Shape) -> Vec<Shape> {
    let mut intersections_done = vec![false; ag.intersections_len];
    let mut shapes = Vec::new();

    while let Some(i) = find_index_false(&intersections_done) {
        let first_intersection = &ag.data[i];
        intersections_done[i] = true;

        let mut merged = Shape {
            start: first_intersection.coord_ptr(),
            curves: Vec::new(),
            color: a.color.clone(),
        };

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

                    merged.curves.push(Curve::new(cp0, cp1, p1));

                    if current.intersect {
                        intersections_done[next] = true;
                        break;
                    }
                }
            } else if ptr::eq(current_shape, bg) && !current.entry {
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
                        intersections_done[next] = true;
                        break;
                    }
                }
            } else if current.entry {
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
                        intersections_done[next] = true;
                        break;
                    }
                }
            } else {
                // !current.entry and shape
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
                        intersections_done[next] = true;
                        break;
                    }
                }
            }

            let next = current.neighbor.unwrap();
            let eq = ptr::eq(current_shape, ag);
            if eq {
                current_shape = bg;
            } else {
                current_shape = ag;
            }
            current = &current_shape.data[next];

            // first interaction is from ag
            if ptr::eq(current, first_intersection)
                || ptr::eq(
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
    use super::{shape_difference, ShapeDifference};
    use common::{pures::Affine, types::Coord, Rgba};

    use crate::{create_circle, shape::Shape, Vgc};

    #[test]
    fn given_two_circle_when_difference_then_new_1() {
        let mut vgc = Vgc::new(Rgba::new(255, 255, 255, 255));

        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.2, 0.2);
        create_circle(&mut vgc, Coord::new(0.2, 0.0), 0.2, 0.2);

        let a = vgc.get_shape(0).expect("Shape should exist");
        let b = vgc.get_shape(1).expect("Shape should exist");

        let merged = shape_difference(&a, &b);
        let merged = match merged {
            ShapeDifference::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 1);
        let merged = &merged[0];
        assert_eq!(merged.curves.len(), 6);

        let steps = 6;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &Coord::new(x, y);
                assert_eq!(
                    merged.contains(&coord),
                    a.contains(&coord) && !b.contains(&coord),
                    "Contains failed at ({}, {})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn given_two_oval_with_no_valid_p_when_difference_then_new_2() {
        let mut shape1 = vec![
            //horizontal oval
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
            //vertical oval
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

        let merged = shape_difference(&a, &b);

        let merged = match merged {
            ShapeDifference::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].curves.len(), 3);
        assert_eq!(merged[1].curves.len(), 3);

        let steps = 5;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &Coord::new(x, y);
                assert_eq!(
                    merged[0].contains(&coord) || merged[1].contains(&coord),
                    a.contains(&coord) && !b.contains(&coord),
                    "Contains failed at ({}, {})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    #[ignore]
    fn given_two_circle_when_difference_then_awithbhole() {
        let mut vgc = Vgc::new(Rgba::new(255, 255, 255, 255));

        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.2, 0.2);
        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.1, 0.1);

        let a = vgc.get_shape(0).expect("Shape should exist");
        let b = vgc.get_shape(1).expect("Shape should exist");

        let merged = shape_difference(&a, &b);

        assert!(
            matches!(merged, ShapeDifference::AWithBHole),
            "ShapeDifference::AWithBHole"
        );
    }

    #[test]
    fn given_two_circle_when_difference_then_a() {
        let mut vgc = Vgc::new(Rgba::new(255, 255, 255, 255));

        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.2, 0.2);
        create_circle(&mut vgc, Coord::new(0.3, 0.3), 0.1, 0.1);

        let a = vgc.get_shape(0).expect("Shape should exist");
        let b = vgc.get_shape(1).expect("Shape should exist");

        let merged = shape_difference(&a, &b);

        assert!(
            matches!(merged, ShapeDifference::A),
            "Should be ShapeDifference::A"
        );
    }

    #[test]
    fn given_two_circle_when_difference_then_ereasea() {
        let mut vgc = Vgc::new(Rgba::new(255, 255, 255, 255));

        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.1, 0.1);
        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.2, 0.2);

        let a = vgc.get_shape(0).expect("Shape should exist");
        let b = vgc.get_shape(1).expect("Shape should exist");

        let merged = shape_difference(&a, &b);

        assert!(
            matches!(merged, ShapeDifference::EraseA),
            "Should be ShapeDifference::EraseA"
        );
    }

    #[test]
    fn given_shape_with_intersections_point_as_b_when_difference_then_valid() {
        //A: M 1 -1 C 1 -1 1 1 1 1 C 1 1 0 0 0 0 C 0 0 -1 -1 -1 -1 C -1 -1 1 -1 1 -1 Z
        /*B: M -0.47455588 -0.47455588
        C -0.47455588 -0.47455588 0 0 0 0
        C 0 0 1 1 1 1
        C 1 1 -1 1 -1 1
        C -1 1 -1 -1 -1 -1
        C -1 -1 -0.68605244 -0.68605244 -0.68605244 -0.68605244
        C -0.6632519 -0.713484 -0.6342824 -0.729898 -0.6026667 -0.730011
        C -0.5288764 -0.7297472 -0.46950188 -0.6406853 -0.46932596 -0.53
        C -0.46935654 -0.51074654 -0.4711784 -0.4921474 -0.47455588 -0.47455588 Z*/

        let coord_a = vec![
            Coord::new(1.0, -1.0),
            Coord::new(1.0, -1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0),
            Coord::new(-1.0, -1.0),
            Coord::new(-1.0, -1.0),
            Coord::new(-1.0, -1.0),
            Coord::new(1.0, -1.0),
            Coord::new(1.0, -1.0),
        ];

        let coord_b = vec![
            Coord::new(-0.47455588, -0.47455588),
            Coord::new(-0.47455588, -0.47455588),
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(-1.0, 1.0),
            Coord::new(-1.0, 1.0),
            Coord::new(-1.0, 1.0),
            Coord::new(-1.0, -1.0),
            Coord::new(-1.0, -1.0),
            Coord::new(-1.0, -1.0),
            Coord::new(-0.68605244, -0.68605244),
            Coord::new(-0.68605244, -0.68605244),
            Coord::new(-0.6632519, -0.713484),
            Coord::new(-0.6342824, -0.729898),
            Coord::new(-0.6026667, -0.730011),
            Coord::new(-0.5288764, -0.7297472),
            Coord::new(-0.46950188, -0.6406853),
            Coord::new(-0.46932596, -0.53),
            Coord::new(-0.46935654, -0.51074654),
            Coord::new(-0.4711784, -0.4921474),
            Coord::new(-0.47455588, -0.47455588),
        ];

        let a = Shape::new_from_path(&coord_a, Affine::identity(), Rgba::black());
        let b = Shape::new_from_path(&coord_b, Affine::identity(), Rgba::black());

        //let intersections = super::find_intersecions(&a, &b);
        //let bg = super::create_shape(&b, intersections.1);
        //bg.print_coords_table();

        let merged = shape_difference(&a, &b);

        assert!(
            matches!(merged, ShapeDifference::New(_)),
            "Should be ShapeDifference::New"
        );

        let merged = match merged {
            ShapeDifference::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 1);

        let steps = 6;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &Coord::new(x, y);
                assert_eq!(
                    merged[0].contains(&coord),
                    a.contains(&coord) && !b.contains(&coord),
                    "Contains failed at ({}, {})",
                    x,
                    y
                );
            }
        }
    }
}
