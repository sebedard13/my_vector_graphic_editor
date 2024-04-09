use super::{create_shape, find_intersecions, mark_entry_exit_points, GreinerShape};
use crate::{curve::Curve, shape::Shape};

pub enum ShapeUnion {
    /// A contains fully B
    A,
    /// B contains fully A
    B,
    /// A and B do not fully contain each other
    /// New shape is created    
    New(Shape),
    /// A and B do not intersect each other
    None,
}

#[allow(dead_code)]
pub fn shape_union(a: &Shape, b: &Shape) -> ShapeUnion {
    let (intersections_a, intersections_b) = find_intersecions(a, b);

    assert_eq!(intersections_a.len(), intersections_b.len());
    assert_eq!(intersections_a.len() % 2, 0); // Shape are closed so we should have an even number of intersections
    if intersections_a.is_empty() && intersections_b.is_empty() {
        if a.contains(&b.start.borrow()) {
            return ShapeUnion::A;
        } else if b.contains(&a.start.borrow()) {
            return ShapeUnion::B;
        } else {
            return ShapeUnion::None;
        }
    }

    let mut ag = create_shape(a, intersections_a);
    let mut bg = create_shape(b, intersections_b);

    mark_entry_exit_points(&mut ag, a, &mut bg, b);

    let merge_shape = do_union(&ag, &bg, a, b);

    ShapeUnion::New(merge_shape)
}

fn do_union(ag: &GreinerShape, bg: &GreinerShape, a: &Shape, _b: &Shape) -> Shape {
    let first_intersection = &ag.data[0];

    let mut merged = Shape {
        start: first_intersection.coord_ptr(),
        curves: Vec::new(),
        color: a.color.clone(),
    };

    let mut current = first_intersection;
    let mut current_shape = ag;
    loop {
        if !current.entry {
            //Remove ! to make the algo A AND B
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

                merged.curves.push(Curve::new(cp0, cp1, p1));

                if current.intersect {
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

    merged
}

#[cfg(test)]
mod test {
    use super::super::{create_shape, find_intersecions, mark_entry_exit_points};
    use super::{shape_union, ShapeUnion};
    use common::{types::Coord, Rgba};

    use crate::{create_circle, Vgc};

    #[test]
    fn given_two_circle_when_union_then_new() {
        let mut vgc = Vgc::new(Rgba::new(255, 255, 255, 255));

        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.2, 0.2);
        create_circle(&mut vgc, Coord::new(0.2, 0.0), 0.2, 0.2);

        let a = vgc.get_shape(0).expect("Shape should exist");
        let b = vgc.get_shape(1).expect("Shape should exist");

        let (i_a, i_b) = find_intersecions(a, b);

        assert_eq!(i_a.len(), 2);
        assert_eq!(i_b.len(), 2);

        let mut ag = create_shape(a, i_a);
        let mut bg = create_shape(b, i_b);

        assert_eq!(ag.len(), 18);
        assert_eq!(bg.len(), 18);

        mark_entry_exit_points(&mut ag, a, &mut bg, b);

        assert_eq!(ag.get(0).entry, false);
        assert_eq!(ag.get(3).entry, true);
        assert_eq!(ag.get(9).entry, false);

        assert_eq!(bg.get(0).entry, false);
        assert_eq!(bg.get(9).entry, true);
        assert_eq!(bg.get(15).entry, false);

        let merged = shape_union(&a, &b);
        let merged = match merged {
            ShapeUnion::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.curves.len(), 8);

        let steps = 5;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &Coord::new(x, y);
                assert_eq!(
                    merged.contains(&coord),
                    a.contains(&coord) || b.contains(&coord),
                    "Contains failed at ({}, {})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn given_two_oval_with_no_valid_p_when_union_then_new() {
        let mut shape1 = vec![
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

        let (i_a, i_b) = find_intersecions(a, b);

        assert_eq!(i_a.len(), 4);
        assert_eq!(i_b.len(), 4);

        let mut ag = create_shape(a, i_a);
        let mut bg = create_shape(b, i_b);

        mark_entry_exit_points(&mut ag, a, &mut bg, b);

        assert_eq!(ag.len(), 18);
        assert_eq!(bg.len(), 18);

        let merged = shape_union(&a, &b);

        let merged = match merged {
            ShapeUnion::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.curves.len(), 4);

        let steps = 5;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &Coord::new(x, y);
                assert_eq!(
                    merged.contains(&coord),
                    a.contains(&coord) || b.contains(&coord),
                    "Contains failed at ({}, {})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn given_two_circle_when_union_then_a() {
        let mut vgc = Vgc::new(Rgba::new(255, 255, 255, 255));

        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.2, 0.2);
        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.1, 0.1);

        let a = vgc.get_shape(0).expect("Shape should exist");
        let b = vgc.get_shape(1).expect("Shape should exist");

        let merged = shape_union(&a, &b);

        assert!(matches!(merged, ShapeUnion::A), "Should be ShapeUnion::A");
    }

    #[test]
    fn given_two_circle_when_union_then_none() {
        let mut vgc = Vgc::new(Rgba::new(255, 255, 255, 255));

        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.2, 0.2);
        create_circle(&mut vgc, Coord::new(0.3, 0.3), 0.1, 0.1);

        let a = vgc.get_shape(0).expect("Shape should exist");
        let b = vgc.get_shape(1).expect("Shape should exist");

        let merged = shape_union(&a, &b);

        assert!(
            matches!(merged, ShapeUnion::None),
            "Should be ShapeUnion::None"
        );
    }
}
