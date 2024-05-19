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

        let steps = 30;
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

        let steps = 30;
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
        //ag.print_coords_table();
        //bg.print_coords_table();

        let merged = shape_difference(&a, &b);

        let merged = match merged {
            ShapeDifference::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 1);

        let steps = 30;
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
        //ag.print_coords_table();
        //bg.print_coords_table();

        let merged = shape_difference(&a, &b);

        let merged = match merged {
            ShapeDifference::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 1);

        let steps = 30;
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
        //ag.print_coords_table();
        //bg.print_coords_table();

        let merged = shape_difference(&a, &b);

        let merged = match merged {
            ShapeDifference::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 1);
       // println!("{}", merged[0].path());

        let steps = 30;
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
    fn bugg3() {
        let a = Shape::quick_from_string("M -0.9530624 -0.9530624 C -0.9530624 -0.9530624 -1 -1 -1 -1 C -1 -1 1 -1 1 -1 C 1 -1 1 1 1 1 C 1 1 0 0 0 0 C 0 0 -0.7462724 -0.7462724 -0.7462724 -0.7462724 C -0.7442392 -0.76013863 -0.7431573 -0.77455163 -0.7431338 -0.78935134 C -0.74330974 -0.9000367 -0.80268425 -0.98909855 -0.8764745 -0.98936236 C -0.90502703 -0.98926026 -0.9314211 -0.975863 -0.9530624 -0.9530624 Z");
        let b = Shape::quick_from_string("M -0.7606782 -0.88851035 C -0.7836163 -0.9487264 -0.8267758 -0.9891847 -0.8764745 -0.98936236 C -0.90502703 -0.98926026 -0.9314211 -0.975863 -0.9530624 -0.9530624 C -0.9530624 -0.9530624 -1 -1 -1 -1 C -1 -1 -1 -0.8646681 -1 -0.8646681 C -1.0062921 -0.8414452 -1.0097728 -0.8160262 -1.0098152 -0.78935134 C -1.0097728 -0.76267624 -1.0062921 -0.73725706 -1 -0.7140342 C -1 -0.7140342 -1 1 -1 1 C -1 1 1 1 1 1 C 1 1 0 0 0 0 C 0 0 -0.6278885 -0.6278885 -0.6278885 -0.6278885 C -0.62381715 -0.6470487 -0.6216068 -0.6674776 -0.6215731 -0.6887016 C -0.62174904 -0.799387 -0.68112355 -0.88844883 -0.7549138 -0.88871264 C -0.7568454 -0.88870573 -0.7587671 -0.88863796 -0.7606782 -0.88851035 Z");

        let intersections = super::find_intersecions(&a, &b);

        let mut ag = super::create_shape(&a, intersections.0);
        let mut bg = super::create_shape(&b, intersections.1);

        super::mark_entry_exit_points(&mut ag, &a, &mut bg, &b);
        //ag.print_coords_table();
       // bg.print_coords_table();

        let merged = shape_difference(&a, &b);

        let merged = match merged {
            ShapeDifference::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 1);
        //println!("{}", merged[0].path());

        let steps = 30;
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
    fn bugg4() {
        let a = Shape::quick_from_string("M -0.27740008 -0.36726904 C -0.24063542 -0.3354444 -0.21502545 -0.27811226 -0.21116556 -0.21116556 C -0.21116556 -0.21116556 0 0 0 0 C 0 0 1 1 1 1 C 1 1 1 -1 1 -1 C 1 -1 -1 -1 -1 -1 C -1 -1 -0.6365884 -0.6365884 -0.6365884 -0.6365884 C -0.6136078 -0.66489893 -0.5841743 -0.68189585 -0.55200005 -0.6820109 C -0.4782098 -0.6817471 -0.41883525 -0.5926852 -0.41865933 -0.48199987 C -0.41868675 -0.4647375 -0.4201541 -0.44800115 -0.42289132 -0.4320454 C -0.40982494 -0.43846637 -0.39585844 -0.44195896 -0.38133335 -0.4420109 C -0.3392175 -0.44186032 -0.3017977 -0.4127832 -0.27740008 -0.36726904 Z");
        let b = Shape::quick_from_string("M -0.44310337 -0.597401 C -0.46721092 -0.64853907 -0.5069147 -0.6818497 -0.55200005 -0.6820109 C -0.5841743 -0.68189585 -0.6136078 -0.66489893 -0.6365884 -0.6365884 C -0.6365884 -0.6365884 -1 -1 -1 -1 C -1 -1 -1 1 -1 1 C -1 1 1 1 1 1 C 1 1 0 0 0 0 C 0 0 -0.21116556 -0.21116556 -0.21116556 -0.21116556 C -0.21502545 -0.27811217 -0.24063537 -0.33544427 -0.27740008 -0.36726904 C -0.28916234 -0.38921174 -0.30395132 -0.40733403 -0.32083404 -0.4202364 C -0.3279281 -0.51536286 -0.37904608 -0.5899601 -0.44310337 -0.597401 Z");

        let intersections = super::find_intersecions(&a, &b);

        let mut ag = super::create_shape(&a, intersections.0);
        let mut bg = super::create_shape(&b, intersections.1);

        super::mark_entry_exit_points(&mut ag, &a, &mut bg, &b);

        let merged = shape_difference(&a, &b);

        let merged = match merged {
            ShapeDifference::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 1);

        let steps = 30;
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
