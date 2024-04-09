use super::{create_shape, find_intersecions, mark_entry_exit_points, GreinerShape};
use crate::{curve::Curve, shape::Shape};

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

    assert_eq!(intersections_a.len(), intersections_b.len());
    assert_eq!(intersections_a.len() % 2, 0); // Shape are closed so we should have an even number of intersections
    if intersections_a.is_empty() && intersections_b.is_empty() {
        if a.contains(&b.start.borrow()) {
            return ShapeIntersection::B;
        } else if b.contains(&a.start.borrow()) {
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

fn do_intersection(ag: &GreinerShape, bg: &GreinerShape, a: &Shape, b: &Shape) -> Vec<Shape> {
    let first_intersection = {
        let mut current = &ag.data[ag.start];
        while !current.intersect {
            let next = &ag.data[current.next.unwrap()];
            current = next;
        }
        current
    };

    let mut merged = Shape {
        start: first_intersection.coord_ptr(),
        curves: Vec::new(),
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

    vec![merged]
}
