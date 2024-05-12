use std::ptr;

use super::{
    create_shape, find_intersecions, mark_entry_exit_points, CoordOfIntersection, GreinerShape,
};
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

    if empty_intersection(&intersections_a) && empty_intersection(&intersections_b) {
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

fn do_difference(ag: &GreinerShape, bg: &GreinerShape, a: &Shape, _b: &Shape) -> Vec<Shape> {
    let mut intersections_done = vec![false; ag.intersections_len];

    for i in 0..ag.intersections_len {
        let current = &ag.data[i];
        if !current.intersect.is_intersection() {
            intersections_done[i] = true;
        }
    }

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

                    if current.intersect.is_intersection() {
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

                    if current.intersect.is_intersection() {
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

                    if current.intersect.is_intersection() {
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

                    if current.intersect.is_intersection() {
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

        let intersections = super::find_intersecions(&a, &b);
        let mut ag = super::create_shape(&a, intersections.0);
        let mut bg = super::create_shape(&b, intersections.1);

        super::mark_entry_exit_points(&mut ag, &a, &mut bg, &b);

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
                let coord = &Coord::new(x + 0.001, y - 0.002);
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

    #[test]
    fn given_shape_with_start_on_common_curves_when_dif_then_valid() {
        /*A: M -0.8041045 -0.8041045 C -0.8041045 -0.8041045 -1 -1 -1 -1
        C -1 -1 1 -1 1 -1 C 1 -1 1 1 1 1
        C 1 1 0 0 0 0 C 0 0 -0.6297539 -0.6297539 -0.6297539 -0.6297539
        C -0.6352249 -0.7330162 -0.69241214 -0.8137597 -0.7626667 -0.814011
        C -0.7771472 -0.8139592 -0.79107255 -0.8104878 -0.8041045 -0.8041045 Z
        //B: M -0.37163284 -0.37163284
        C -0.37163284 -0.37163284 0 0 0 0
        C 0 0 1 1 1 1
        C 1 1 -1 1 -1 1
        C -1 1 -1 -1 -1 -1
        C -1 -1 -0.8041045 -0.8041045 -0.8041045 -0.8041045
        C -0.79107255 -0.8104878 -0.7771472 -0.8139592 -0.7626667 -0.814011
        C -0.69241214 -0.8137597 -0.6352249 -0.7330162 -0.6297539 -0.6297539
        C -0.6297539 -0.6297539 -0.5797497 -0.5797497 -0.5797497 -0.5797497
        C -0.5577743 -0.6037415 -0.5306951 -0.617906 -0.50133336 -0.618011
        C -0.4275431 -0.61774707 -0.36816856 -0.5286853 -0.36799264 -0.41799992
        C -0.368018 -0.40202662 -0.3692763 -0.38650364 -0.37163284 -0.37163284 Z*/

        let coord_a = vec![
            Coord::new(-0.8041045, -0.8041045),
            //
            Coord::new(-0.8041045, -0.8041045),
            Coord::new(-1.0, -1.0),
            Coord::new(-1.0, -1.0),
            //
            Coord::new(-1.0, -1.0),
            Coord::new(1.0, -1.0),
            Coord::new(1.0, -1.0),
            //
            Coord::new(1.0, -1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            //
            Coord::new(1.0, 1.0),
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0),
            //
            Coord::new(0.0, 0.0),
            Coord::new(-0.6297539, -0.6297539),
            Coord::new(-0.6297539, -0.6297539),
            //
            Coord::new(-0.6352249, -0.7330162),
            Coord::new(-0.69241214, -0.8137597),
            Coord::new(-0.7626667, -0.814011),
            //
            Coord::new(-0.7771472, -0.8139592),
            Coord::new(-0.79107255, -0.8104878),
            Coord::new(-0.8041045, -0.8041045),
        ];

        let coord_b = vec![
            Coord::new(-0.37163284, -0.37163284),
            //
            Coord::new(-0.37163284, -0.37163284),
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0),
            //
            Coord::new(0.0, 0.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            //
            Coord::new(1.0, 1.0),
            Coord::new(-1.0, 1.0),
            Coord::new(-1.0, 1.0),
            //
            Coord::new(-1.0, 1.0),
            Coord::new(-1.0, -1.0),
            Coord::new(-1.0, -1.0),
            //
            Coord::new(-1.0, -1.0),
            Coord::new(-0.8041045, -0.8041045),
            Coord::new(-0.8041045, -0.8041045),
            //
            Coord::new(-0.79107255, -0.8104878),
            Coord::new(-0.7771472, -0.8139592),
            Coord::new(-0.7626667, -0.814011),
            //
            Coord::new(-0.69241214, -0.8137597),
            Coord::new(-0.6352249, -0.7330162),
            Coord::new(-0.6297539, -0.6297539),
            //
            Coord::new(-0.6297539, -0.6297539),
            Coord::new(-0.5797497, -0.5797497),
            Coord::new(-0.5797497, -0.5797497),
            //
            Coord::new(-0.5577743, -0.6037415),
            Coord::new(-0.5306951, -0.617906),
            Coord::new(-0.50133336, -0.618011),
            //
            Coord::new(-0.4275431, -0.61774707),
            Coord::new(-0.36816856, -0.5286853),
            Coord::new(-0.36799264, -0.41799992),
            //
            Coord::new(-0.368018, -0.40202662),
            Coord::new(-0.3692763, -0.38650364),
            Coord::new(-0.37163284, -0.37163284),
        ];

        let a = Shape::new_from_path(&coord_a, Affine::identity(), Rgba::black());
        let b = Shape::new_from_path(&coord_b, Affine::identity(), Rgba::black());

        let intersections = super::find_intersecions(&a, &b);

        let mut ag = super::create_shape(&a, intersections.0);
        let mut bg = super::create_shape(&b, intersections.1);

        super::mark_entry_exit_points(&mut ag, &a, &mut bg, &b);
        ag.print_coords_table();
        bg.print_coords_table();

        let merged = shape_difference(&a, &b);

        let merged = match merged {
            ShapeDifference::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 1);

        let steps = 6;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &Coord::new(x + 0.001, y - 0.002);
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

    #[test]
    fn buggg() {
        /*A: M -0.8041045 -0.8041045 C -0.8041045 -0.8041045 -1 -1 -1 -1
        C -1 -1 1 -1 1 -1 C 1 -1 1 1 1 1
        C 1 1 0 0 0 0 C 0 0 -0.6297539 -0.6297539 -0.6297539 -0.6297539
        C -0.6352249 -0.7330162 -0.69241214 -0.8137597 -0.7626667 -0.814011
        C -0.7771472 -0.8139592 -0.79107255 -0.8104878 -0.8041045 -0.8041045 Z*/
        /*B: M -0.37163284 -0.37163284
        C -0.37163284 -0.37163284 0 0 0 0
        C 0 0 1 1 1 1
        C 1 1 -1 1 -1 1
        C -1 1 -1 -1 -1 -1
        C -1 -1 -0.8041045 -0.8041045 -0.8041045 -0.8041045
        C -0.79107255 -0.8104878 -0.7771472 -0.8139592 -0.7626667 -0.814011
        C -0.69241214 -0.8137597 -0.6352249 -0.7330162 -0.6297539 -0.6297539
        C -0.6297539 -0.6297539 -0.5797497 -0.5797497 -0.5797497 -0.5797497
        C -0.5577743 -0.6037415 -0.5306951 -0.617906 -0.50133336 -0.618011
        C -0.4275431 -0.61774707 -0.36816856 -0.5286853 -0.36799264 -0.41799992
        C -0.368018 -0.40202662 -0.3692763 -0.38650364 -0.37163284 -0.37163284 Z*/

        let coord_a = vec![
            Coord::new(-0.8041045, -0.8041045),
            //
            Coord::new(-0.8041045, -0.8041045),
            Coord::new(-1.0, -1.0),
            Coord::new(-1.0, -1.0),
            //
            Coord::new(-1.0, -1.0),
            Coord::new(1.0, -1.0),
            Coord::new(1.0, -1.0),
            //
            Coord::new(1.0, -1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            //
            Coord::new(1.0, 1.0),
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0),
            //
            Coord::new(0.0, 0.0),
            Coord::new(-0.6297539, -0.6297539),
            Coord::new(-0.6297539, -0.6297539),
            //
            Coord::new(-0.6352249, -0.7330162),
            Coord::new(-0.69241214, -0.8137597),
            Coord::new(-0.7626667, -0.814011),
            //
            Coord::new(-0.7771472, -0.8139592),
            Coord::new(-0.79107255, -0.8104878),
            Coord::new(-0.8041045, -0.8041045),
        ];

        let coord_b = vec![
            Coord::new(-0.37163284, -0.37163284),
            //
            Coord::new(-0.37163284, -0.37163284),
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0),
            //
            Coord::new(0.0, 0.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            //
            Coord::new(1.0, 1.0),
            Coord::new(-1.0, 1.0),
            Coord::new(-1.0, 1.0),
            //
            Coord::new(-1.0, 1.0),
            Coord::new(-1.0, -1.0),
            Coord::new(-1.0, -1.0),
            //
            Coord::new(-1.0, -1.0),
            Coord::new(-0.8041045, -0.8041045),
            Coord::new(-0.8041045, -0.8041045),
            //
            Coord::new(-0.79107255, -0.8104878),
            Coord::new(-0.7771472, -0.8139592),
            Coord::new(-0.7626667, -0.814011),
            //
            Coord::new(-0.69241214, -0.8137597),
            Coord::new(-0.6352249, -0.7330162),
            Coord::new(-0.6297539, -0.6297539),
            //
            Coord::new(-0.6297539, -0.6297539),
            Coord::new(-0.5797497, -0.5797497),
            Coord::new(-0.5797497, -0.5797497),
            //
            Coord::new(-0.5577743, -0.6037415),
            Coord::new(-0.5306951, -0.617906),
            Coord::new(-0.50133336, -0.618011),
            //
            Coord::new(-0.4275431, -0.61774707),
            Coord::new(-0.36816856, -0.5286853),
            Coord::new(-0.36799264, -0.41799992),
            //
            Coord::new(-0.368018, -0.40202662),
            Coord::new(-0.3692763, -0.38650364),
            Coord::new(-0.37163284, -0.37163284),
        ];

        let a = Shape::new_from_path(&coord_a, Affine::identity(), Rgba::black());
        let b = Shape::new_from_path(&coord_b, Affine::identity(), Rgba::black());

        let intersections = super::find_intersecions(&a, &b);

        let mut ag = super::create_shape(&a, intersections.0);
        let mut bg = super::create_shape(&b, intersections.1);

        super::mark_entry_exit_points(&mut ag, &a, &mut bg, &b);
        ag.print_coords_table();
        bg.print_coords_table();

        let merged = shape_difference(&a, &b);

        let merged = match merged {
            ShapeDifference::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 1);

        let steps = 6;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &Coord::new(x + 0.001, y - 0.002);
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

    #[test]
    fn bugg2() {
        let a = Shape::quick_from_string(
            "M -0.45359507 -0.45359507 C -0.45359507 -0.45359507 0 0 0 0 
        C 0 0 1 1 1 1 C 1 1 1 -1 1 -1 C 1 -1 -1 -1 -1 -1 
        C -1 -1 -0.7198505 -0.7198505 -0.7198505 -0.7198505 
        C -0.69678396 -0.7486032 -0.6671195 -0.7658949 -0.6346667 -0.7660109 
        C -0.575112 -0.765798 -0.5249474 -0.70774376 -0.50771606 -0.62716335 
        C -0.47476742 -0.5908155 -0.4534313 -0.5322852 -0.453326 -0.46599996 
        C -0.45333263 -0.4618332 -0.45342314 -0.45769712 -0.45359507 -0.45359507 Z",
        );
        let b = Shape::quick_from_string("M -0.84374976 -0.84374976 C -0.84374976 -0.84374976 -1 -1 -1 -1 
        C -1 -1 -1 1 -1 1 C -1 1 1 1 1 1 C 1 1 0 0 0 0 C 0 0 -0.45359507 -0.45359507 -0.45359507 -0.45359507 
        C -0.45342314 -0.45769712 -0.45333263 -0.4618332 -0.453326 -0.46599996 
        C -0.4534313 -0.5322852 -0.47476742 -0.5908155 -0.50771606 -0.62716335
         C -0.5249475 -0.7077439 -0.57511204 -0.765798 -0.6346667 -0.7660109 
         C -0.6378605 -0.76599944 -0.6410273 -0.76582164 -0.64416337 -0.76548296 
         C -0.6652278 -0.8342496 -0.711435 -0.88181823 -0.7653334 -0.88201094 
         C -0.79469514 -0.881906 -0.8217743 -0.8677416 -0.84374976 -0.84374976 Z");

        let intersections = super::find_intersecions(&a, &b);

        let mut ag = super::create_shape(&a, intersections.0);
        let mut bg = super::create_shape(&b, intersections.1);

        super::mark_entry_exit_points(&mut ag, &a, &mut bg, &b);
        ag.print_coords_table();
        bg.print_coords_table();

        let merged = shape_difference(&a, &b);

        let merged = match merged {
            ShapeDifference::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 1);
        println!("{}", merged[0].path());

        let steps = 6;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &Coord::new(x + 0.001, y - 0.002);
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

    #[test]
    #[ignore]
    fn bugg3() {
        let a = Shape::quick_from_string("M -0.86278796 -1 C -0.8567189 -0.9771342 -0.85336757 -0.95217156 -0.85332596 -0.92599994 C -0.85336244 -0.9030905 -0.8559348 -0.8811074 -0.8606463 -0.8606463 C -0.8606463 -0.8606463 -0.67049706 -0.67049706 -0.67049706 -0.67049706 C -0.67049706 -0.67049706 -0.55413175 -0.55413175 -0.55413175 -0.55413175 C -0.55413175 -0.55413175 0 0 0 0 C 0 0 1 1 1 1 C 1 1 -0.5684262 1.0000001 -0.5684262 1.0000001 C -0.58733326 1.0759544 -0.63597876 1.1298062 -0.6933334 1.1300112 C -0.7138853 1.1299378 -0.733319 1.1229761 -0.7506668 1.1105773 C -0.76801455 1.122976 -0.78744817 1.1299378 -0.8080001 1.1300112 C -0.84872985 1.1298656 -0.88506764 1.1026661 -0.90948224 1.0597093 C -0.9277851 1.0738978 -0.94858015 1.0819322 -0.97066677 1.0820111 C -1.044457 1.0817473 -1.1038315 0.99268544 -1.1040075 0.8820001 C -1.1038555 0.78644246 -1.059581 0.7070016 -1 0.6869019 C -1 0.6869019 -1 -0.7270134 -1 -0.7270134 C -1.0674807 -0.7371993 -1.1198422 -0.8220668 -1.1200074 -0.92599994 C -1.1198314 -1.0366853 -1.060457 -1.1257471 -0.9866667 -1.126011 C -0.93032414 -1.1258094 -0.8823861 -1.0738382 -0.86278796 -1 Z");
        let b = Shape::quick_from_string("M -0.14272948 -0.14272948 C -0.14272948 -0.14272948 0 0 0 0 C 0 0 1 1 1 1 C 1 1 1 -1 1 -1 C 1 -1 -0.86278796 -1 -0.86278796 -1 C -0.8567189 -0.97713435 -0.85336757 -0.95217156 -0.85332596 -0.92599994 C -0.85336244 -0.9030905 -0.8559348 -0.8811074 -0.8606463 -0.8606463 C -0.86064637 -0.86064637 -0.7811588 -0.7811588 -0.67049706 -0.67049706 C -0.65005815 -0.6037906 -0.60595876 -0.55703574 -0.55413175 -0.55413175 C -0.55413175 -0.55413175 -0.36246973 -0.36246973 -0.36246973 -0.36246973 C -0.37089944 -0.33624113 -0.37563822 -0.30680203 -0.37568772 -0.27565014 C -0.3755118 -0.16496477 -0.31613725 -0.07590297 -0.242347 -0.0756391 C -0.20264174 -0.07578108 -0.16711031 -0.101633 -0.14272948 -0.14272948 Z");

        let intersections = super::find_intersecions(&a, &b);

        let mut ag = super::create_shape(&a, intersections.0);
        let mut bg = super::create_shape(&b, intersections.1);

        super::mark_entry_exit_points(&mut ag, &a, &mut bg, &b);
        ag.print_coords_table();
        bg.print_coords_table();

        let merged = shape_difference(&a, &b);

        let merged = match merged {
            ShapeDifference::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 1);
        println!("{}", merged[0].path());

        let steps = 6;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &Coord::new(x + 0.001, y - 0.002);
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
