use std::ptr;

use anyhow::Error;

use super::{
    create_shape, find_intersecions, mark_entry_exit_points, CoordOfIntersection, GreinerShape,
};
use crate::scene::shape::Shape;

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
    match try_shape_difference(a, b) {
        Ok(difference) => difference,
        Err(e) => {
            log::error!(
                "Error in shape_difference a: {:?} b: {:?} error: {}",
                a,
                b,
                e
            );
            ShapeDifference::A
        }
    }
}

fn try_shape_difference(a: &Shape, b: &Shape) -> Result<ShapeDifference, Error> {
    let (intersections_a, intersections_b) = find_intersecions(a, b);

    if empty_intersection(&intersections_a) && empty_intersection(&intersections_b) {
        if a.contains(&b.path[0].coord) {
            return Ok(ShapeDifference::AWithBHole);
        } else if b.contains(&a.path[0].coord) {
            return Ok(ShapeDifference::EraseA);
        } else {
            return Ok(ShapeDifference::A);
        }
    }

    let mut ag = create_shape(a, intersections_a);
    let mut bg = create_shape(b, intersections_b);

    mark_entry_exit_points(&mut ag, a, &mut bg, b);

    let merged_shapes = do_difference(&ag, &bg, a, b);

    Ok(ShapeDifference::New(merged_shapes))
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
            id: a.id,
            path: vec![first_intersection.coord_ptr()],
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

                    merged.path.append(&mut vec![cp0, cp1, p1]);

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

                    merged.path.append(&mut vec![cp0, cp1, p1]);

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

                    merged.path.append(&mut vec![cp0, cp1, p1]);

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

                    merged.path.append(&mut vec![cp0, cp1, p1]);

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
    use common::{
        pures::Affine,
        types::{Coord, Length2d},
    };
    use log::LevelFilter;

    use crate::{scene::shape::Shape, DbCoord};

    #[test]
    fn given_two_circle_when_difference_then_new_1() {
        let a = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.2, 0.2));
        let b = Shape::new_circle(Coord::new(0.2, 0.0), Length2d::new(0.2, 0.2));

        let merged = shape_difference(&a, &b);
        let merged = match merged {
            ShapeDifference::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 1);
        let merged = &merged[0];
        assert_eq!(merged.curves_len(), 6);

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
            //vertical oval
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

        let merged = shape_difference(&a, &b);

        let merged = match merged {
            ShapeDifference::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].curves_len(), 3);
        assert_eq!(merged[1].curves_len(), 3);

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
    fn given_two_circle_when_difference_then_awithhole() {
        let a = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.2, 0.2));
        let b = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.1, 0.1));

        let merged = shape_difference(&a, &b);

        assert!(
            matches!(merged, ShapeDifference::AWithBHole),
            "ShapeDifference::AWithBHole"
        );
    }

    #[test]
    fn given_two_circle_when_difference_then_a() {
        let a = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.2, 0.2));
        let b = Shape::new_circle(Coord::new(0.3, 0.3), Length2d::new(0.1, 0.1));

        let merged = shape_difference(&a, &b);

        assert!(
            matches!(merged, ShapeDifference::A),
            "Should be ShapeDifference::A"
        );
    }

    #[test]
    fn given_two_circle_when_difference_then_ereasea() {
        let a = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.1, 0.1));
        let b = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.2, 0.2));

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
            DbCoord::new(1.0, -1.0),
            DbCoord::new(1.0, -1.0),
            DbCoord::new(1.0, 1.0),
            DbCoord::new(1.0, 1.0),
            DbCoord::new(1.0, 1.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(1.0, -1.0),
            DbCoord::new(1.0, -1.0),
        ];

        let coord_b = vec![
            DbCoord::new(-0.47455588, -0.47455588),
            DbCoord::new(-0.47455588, -0.47455588),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(1.0, 1.0),
            DbCoord::new(1.0, 1.0),
            DbCoord::new(1.0, 1.0),
            DbCoord::new(-1.0, 1.0),
            DbCoord::new(-1.0, 1.0),
            DbCoord::new(-1.0, 1.0),
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(-0.68605244, -0.68605244),
            DbCoord::new(-0.68605244, -0.68605244),
            DbCoord::new(-0.6632519, -0.713484),
            DbCoord::new(-0.6342824, -0.729898),
            DbCoord::new(-0.6026667, -0.730011),
            DbCoord::new(-0.5288764, -0.7297472),
            DbCoord::new(-0.46950188, -0.6406853),
            DbCoord::new(-0.46932596, -0.53),
            DbCoord::new(-0.46935654, -0.51074654),
            DbCoord::new(-0.4711784, -0.4921474),
            DbCoord::new(-0.47455588, -0.47455588),
        ];

        let a = Shape::new_from_path(coord_a, Affine::identity());
        let b = Shape::new_from_path(coord_b, Affine::identity());

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
            DbCoord::new(-0.8041045, -0.8041045),
            //
            DbCoord::new(-0.8041045, -0.8041045),
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(-1.0, -1.0),
            //
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(1.0, -1.0),
            DbCoord::new(1.0, -1.0),
            //
            DbCoord::new(1.0, -1.0),
            DbCoord::new(1.0, 1.0),
            DbCoord::new(1.0, 1.0),
            //
            DbCoord::new(1.0, 1.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            //
            DbCoord::new(0.0, 0.0),
            DbCoord::new(-0.6297539, -0.6297539),
            DbCoord::new(-0.6297539, -0.6297539),
            //
            DbCoord::new(-0.6352249, -0.7330162),
            DbCoord::new(-0.69241214, -0.8137597),
            DbCoord::new(-0.7626667, -0.814011),
            //
            DbCoord::new(-0.7771472, -0.8139592),
            DbCoord::new(-0.79107255, -0.8104878),
            DbCoord::new(-0.8041045, -0.8041045),
        ];

        let coord_b = vec![
            DbCoord::new(-0.37163284, -0.37163284),
            //
            DbCoord::new(-0.37163284, -0.37163284),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            //
            DbCoord::new(0.0, 0.0),
            DbCoord::new(1.0, 1.0),
            DbCoord::new(1.0, 1.0),
            //
            DbCoord::new(1.0, 1.0),
            DbCoord::new(-1.0, 1.0),
            DbCoord::new(-1.0, 1.0),
            //
            DbCoord::new(-1.0, 1.0),
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(-1.0, -1.0),
            //
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(-0.8041045, -0.8041045),
            DbCoord::new(-0.8041045, -0.8041045),
            //
            DbCoord::new(-0.79107255, -0.8104878),
            DbCoord::new(-0.7771472, -0.8139592),
            DbCoord::new(-0.7626667, -0.814011),
            //
            DbCoord::new(-0.69241214, -0.8137597),
            DbCoord::new(-0.6352249, -0.7330162),
            DbCoord::new(-0.6297539, -0.6297539),
            //
            DbCoord::new(-0.6297539, -0.6297539),
            DbCoord::new(-0.5797497, -0.5797497),
            DbCoord::new(-0.5797497, -0.5797497),
            //
            DbCoord::new(-0.5577743, -0.6037415),
            DbCoord::new(-0.5306951, -0.617906),
            DbCoord::new(-0.50133336, -0.618011),
            //
            DbCoord::new(-0.4275431, -0.61774707),
            DbCoord::new(-0.36816856, -0.5286853),
            DbCoord::new(-0.36799264, -0.41799992),
            //
            DbCoord::new(-0.368018, -0.40202662),
            DbCoord::new(-0.3692763, -0.38650364),
            DbCoord::new(-0.37163284, -0.37163284),
        ];

        let a = Shape::new_from_path(coord_a, Affine::identity());
        let b = Shape::new_from_path(coord_b, Affine::identity());

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
            DbCoord::new(-0.8041045, -0.8041045),
            //
            DbCoord::new(-0.8041045, -0.8041045),
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(-1.0, -1.0),
            //
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(1.0, -1.0),
            DbCoord::new(1.0, -1.0),
            //
            DbCoord::new(1.0, -1.0),
            DbCoord::new(1.0, 1.0),
            DbCoord::new(1.0, 1.0),
            //
            DbCoord::new(1.0, 1.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            //
            DbCoord::new(0.0, 0.0),
            DbCoord::new(-0.6297539, -0.6297539),
            DbCoord::new(-0.6297539, -0.6297539),
            //
            DbCoord::new(-0.6352249, -0.7330162),
            DbCoord::new(-0.69241214, -0.8137597),
            DbCoord::new(-0.7626667, -0.814011),
            //
            DbCoord::new(-0.7771472, -0.8139592),
            DbCoord::new(-0.79107255, -0.8104878),
            DbCoord::new(-0.8041045, -0.8041045),
        ];

        let coord_b = vec![
            DbCoord::new(-0.37163284, -0.37163284),
            //
            DbCoord::new(-0.37163284, -0.37163284),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            //
            DbCoord::new(0.0, 0.0),
            DbCoord::new(1.0, 1.0),
            DbCoord::new(1.0, 1.0),
            //
            DbCoord::new(1.0, 1.0),
            DbCoord::new(-1.0, 1.0),
            DbCoord::new(-1.0, 1.0),
            //
            DbCoord::new(-1.0, 1.0),
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(-1.0, -1.0),
            //
            DbCoord::new(-1.0, -1.0),
            DbCoord::new(-0.8041045, -0.8041045),
            DbCoord::new(-0.8041045, -0.8041045),
            //
            DbCoord::new(-0.79107255, -0.8104878),
            DbCoord::new(-0.7771472, -0.8139592),
            DbCoord::new(-0.7626667, -0.814011),
            //
            DbCoord::new(-0.69241214, -0.8137597),
            DbCoord::new(-0.6352249, -0.7330162),
            DbCoord::new(-0.6297539, -0.6297539),
            //
            DbCoord::new(-0.6297539, -0.6297539),
            DbCoord::new(-0.5797497, -0.5797497),
            DbCoord::new(-0.5797497, -0.5797497),
            //
            DbCoord::new(-0.5577743, -0.6037415),
            DbCoord::new(-0.5306951, -0.617906),
            DbCoord::new(-0.50133336, -0.618011),
            //
            DbCoord::new(-0.4275431, -0.61774707),
            DbCoord::new(-0.36816856, -0.5286853),
            DbCoord::new(-0.36799264, -0.41799992),
            //
            DbCoord::new(-0.368018, -0.40202662),
            DbCoord::new(-0.3692763, -0.38650364),
            DbCoord::new(-0.37163284, -0.37163284),
        ];

        let a = Shape::new_from_path(coord_a, Affine::identity());
        let b = Shape::new_from_path(coord_b, Affine::identity());

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
        let _ = env_logger::builder()
            .filter_level(LevelFilter::max())
            .is_test(true)
            .try_init();
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
    #[ignore]
    fn bugg4() {
        let a = Shape::quick_from_string("M -0.59370655 -0.59370655 C -0.59370655 -0.59370655 0 0 0 0 C 0 0 1 1 1 1 C 1 1 1 -1 1 -1 C 1 -1 -1 -1 -1 -1 C -1 -1 -0.95007277 -0.95007277 -0.95007277 -0.95007277 C -0.92767864 -0.9757056 -0.89968127 -0.99093807 -0.86922866 -0.9910469 C -0.8064482 -0.99082243 -0.7541027 -0.92632127 -0.7397714 -0.838912 C -0.7343916 -0.8399141 -0.72890997 -0.8404387 -0.72334635 -0.84045863 C -0.6495561 -0.8401948 -0.5901816 -0.75113297 -0.59000564 -0.6404476 C -0.5900312 -0.6243404 -0.5913105 -0.6086912 -0.59370655 -0.59370655 Z ");
        let b = Shape::quick_from_string("M -0.44796872 -0.44796872 C -0.44796872 -0.44796872 0 0 0 0 C 0 0 1 1 1 1 C 1 1 -1 1 -1 1 C -1 1 -1 -0.75204337 -1 -0.75204337 C -1.0016655 -0.7646467 -1.0025481 -0.77768403 -1.0025693 -0.7910359 C -1.0025481 -0.80438733 -1.0016656 -0.81742424 -1 -0.8300302 C -1 -0.8300302 -1 -1 -1 -1 C -1 -1 -0.95007277 -0.95007277 -0.95007277 -0.95007277 C -0.92767864 -0.9757056 -0.89968127 -0.99093807 -0.86922866 -0.9910469 C -0.8064482 -0.99082243 -0.7541027 -0.92632127 -0.7397714 -0.838912 C -0.7343916 -0.8399141 -0.72890997 -0.8404387 -0.72334635 -0.84045863 C -0.6694192 -0.84026587 -0.6231915 -0.7926467 -0.6021426 -0.7238206 C -0.5917414 -0.72773457 -0.580848 -0.7298303 -0.56962085 -0.72987044 C -0.4958306 -0.7296066 -0.43645605 -0.6405448 -0.43628013 -0.5298594 C -0.43632656 -0.5006445 -0.44049722 -0.47293606 -0.44796872 -0.44796872 Z");

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
    #[ignore]
    fn stackoverflow() {
        let a = Shape::quick_from_string("M -0.8835917 -1 C -0.86537874 -0.9787828 -0.8509498 -0.9506341 -0.8420334 -0.9181458 C -0.8336186 -0.9206512 -0.8249145 -0.921979 -0.8160001 -0.9220109 C -0.7719526 -0.9218534 -0.7330419 -0.8900552 -0.70879275 -0.8409032 C -0.67153895 -0.8350525 -0.6389346 -0.8064316 -0.6172402 -0.7644308 C -0.59797806 -0.7806542 -0.5757348 -0.78992605 -0.55200005 -0.7900109 C -0.49558574 -0.7898092 -0.44759732 -0.7377055 -0.4280466 -0.6637172 C -0.42151093 -0.66520435 -0.41481698 -0.6659866 -0.408 -0.666011 C -0.3647655 -0.66585636 -0.32647976 -0.63521826 -0.3021503 -0.5876076 C -0.23817244 -0.57270294 -0.1894852 -0.49021432 -0.18932594 -0.38999987 C -0.1893518 -0.37372667 -0.19065729 -0.35792086 -0.1930998 -0.34279525 C -0.18060592 -0.3123606 -0.17338708 -0.27648753 -0.17332591 -0.23799992 C -0.17333159 -0.23443258 -0.17339873 -0.23088768 -0.1735259 -0.22736734 C -0.16652308 -0.22908157 -0.15933271 -0.22998467 -0.15199995 -0.2300109 C -0.11428007 -0.22987601 -0.080327034 -0.20653805 -0.05612203 -0.16896874 C -0.051698998 -0.1696418 -0.047210164 -0.1699947 -0.042666674 -0.17001095 C 0.0074478593 -0.16983174 0.050913252 -0.12869515 0.073701404 -0.067642756 C 0.10644919 -0.060070023 0.13520065 -0.03479246 0.15550244 0.0015108995 C 0.18445463 0.019695014 0.20817101 0.052693985 0.22248459 0.09425837 C 0.24578059 0.06424448 0.27609444 0.046107985 0.30933332 0.045989126 C 0.37678647 0.046230335 0.43219358 0.12067241 0.4413471 0.21800324 C 0.4417863 0.21799536 0.44222617 0.21799064 0.44266653 0.21798906 C 0.51611984 0.21825172 0.5752888 0.30650324 0.57600117 0.41648445 C 0.62165934 0.44819075 0.65321255 0.5173561 0.6533407 0.59800017 C 0.6533127 0.61561716 0.651785 0.63268626 0.6489366 0.6489366 C 0.6489366 0.6489366 1 1 1 1 C 1 1 1 -1 1 -1 C 1 -1 -0.8835917 -1 -0.8835917 -1 Z");
        let b = Shape::quick_from_string("M -0.32234216 -0.6192571 C -0.31486663 -0.60985094 -0.30809224 -0.5992355 -0.3021503 -0.5876076 C -0.23817244 -0.57270294 -0.1894852 -0.49021432 -0.18932594 -0.38999987 C -0.1893518 -0.37372667 -0.19065729 -0.35792086 -0.1930998 -0.34279525 C -0.18060601 -0.312361 -0.17338708 -0.2764877 -0.17332591 -0.23799992 C -0.17333157 -0.23443243 -0.17339875 -0.23088738 -0.1735259 -0.22736734 C -0.16652308 -0.22908157 -0.15933271 -0.22998467 -0.15199995 -0.2300109 C -0.114279784 -0.22987601 -0.08032653 -0.2065377 -0.05612203 -0.16896874 C -0.051698998 -0.1696418 -0.047210164 -0.1699947 -0.042666674 -0.17001095 C 0.0074478593 -0.16983174 0.050913252 -0.12869515 0.073701404 -0.067642756 C 0.10644919 -0.060070023 0.13520065 -0.03479246 0.15550244 0.0015108995 C 0.18445517 0.019695984 0.20817098 0.05269444 0.22248459 0.09425837 C 0.24578059 0.06424448 0.27609444 0.046107985 0.30933332 0.045989126 C 0.37678647 0.046230335 0.43219358 0.12067241 0.4413471 0.21800324 C 0.4417863 0.21799536 0.44222617 0.21799064 0.44266653 0.21798906 C 0.51611984 0.21825172 0.5752888 0.30650324 0.57600117 0.41648445 C 0.62165934 0.44819075 0.65321255 0.5173561 0.6533407 0.59800017 C 0.6533127 0.61561716 0.651785 0.63268626 0.6489366 0.6489366 C 0.6489366 0.6489366 1 1 1 1 C 1 1 -0.63145053 0.99999994 -0.63145053 0.99999994 C -0.655508 1.0357893 -0.68863815 1.0578799 -0.72533333 1.0580112 C -0.7620286 1.0578799 -0.79515874 1.0357893 -0.81921697 1 C -0.81921697 1 -1 1 -1 1 C -1 1 -1 0.7940968 -1 0.7940968 C -1.0513382 0.7662768 -1.0878694 0.69278514 -1.0880075 0.60600007 C -1.0878694 0.519215 -1.0513382 0.44572335 -1 0.41790286 C -1 0.41790286 -0.99999994 -0.42887396 -0.99999994 -0.42887396 C -1.0568707 -0.45179418 -1.0985267 -0.52931607 -1.098674 -0.62199986 C -1.0986093 -0.66270494 -1.0905385 -0.7004857 -1.0766886 -0.7320006 C -1.0905386 -0.76351565 -1.0986094 -0.8012956 -1.098674 -0.84199995 C -1.0984981 -0.9526853 -1.0391235 -1.0417471 -0.96533334 -1.042011 C -0.9095383 -1.0418115 -0.8619852 -0.9908432 -0.8420334 -0.9181458 C -0.83361876 -0.92065114 -0.82491463 -0.921979 -0.8160001 -0.9220109 C -0.7719526 -0.9218534 -0.7330419 -0.8900552 -0.70879275 -0.8409032 C -0.6715387 -0.8350525 -0.6389342 -0.8064313 -0.6172402 -0.7644308 C -0.59797806 -0.7806542 -0.5757348 -0.78992605 -0.55200005 -0.7900109 C -0.51814187 -0.7898899 -0.48731875 -0.7710736 -0.46385682 -0.740051 C -0.4578051 -0.74132115 -0.45162162 -0.7419885 -0.44533348 -0.74201095 C -0.38982332 -0.7418125 -0.34247112 -0.6913623 -0.32234216 -0.6192571 Z");

        let intersections = super::find_intersecions(&a, &b);

        let mut ag = super::create_shape(&a, intersections.0);
        let mut bg = super::create_shape(&b, intersections.1);

        super::mark_entry_exit_points(&mut ag, &a, &mut bg, &b);
        ag.print_coords_table();
        bg.print_coords_table();

        /*let merged = shape_difference(&a, &b);

        let merged = match merged {
            ShapeDifference::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.len(), 1);

        let steps = 30;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &DbCoord::new(x + 0.001, y - 0.002);
                assert_eq!(
                    merged[0].contains(&coord),
                    a.contains(&coord) && !b.contains(&coord),
                    "Contains failed at ({}, {})",
                    x,
                    y
                );
            }
        }*/
    }

    #[test]
    fn given_squares_2i_1ci_when_difference_then_new() {
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
                assert_ne!("M 0 383 C 0 383 0 360 0 360 C 0 338.3812 0 0 0 0 C 0 0 559 0 559 0 C 559 0 559 383 559 383 C 559 383 45 383 45 383 C 45 383 45 405 45 405 C 45 405 0 405 0 405 C 0 405 0 383 0 383 Z", merged[0].path());
            }
            _ => panic!("Should be a new shape"),
        };
    }

    #[test]
    fn given_edgecase_common_i_when_difference_then_new() {
        let a = Shape::quick_from_string(
            "M 0 0 C 0 0 0 1 0 1 C 0 1 1 1 1 1 C 2 1 2 0 1 0 C 1 0 0 0 0 0 Z",
        );
        let b = Shape::quick_from_string(
            "M -1 -1 C -1 -1 -1 1 -1 1 C -1 1 1 1 1 1 C 1 0 1 -1 1 -1 C 1 -1 -1 -1 -1 -1 Z",
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
                assert_eq!(
                    merged[0].path(),
                    "Mno 0 1 1 C 2 1 2 0 1 0 C 1 0 1 1 1 1 C 1 1 0 1 0 1 Z"
                );
            }
            _ => panic!("Should be a new shape"),
        };
    }
}
