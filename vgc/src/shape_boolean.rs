/*
Implementation of boolean operations on shapes.
For Shape A and B
Union : A OR B
Intersection : A AND B
Difference : A NOR B
Symmetric Difference : A XOR B
*/

use std::{cell::RefCell, rc::Rc};

use crate::{
    curve::{add_smooth_result, Curve},
    curve2::intersection,
    shape::Shape,
};

pub fn union(a: &Shape, b: &Shape) -> Option<Shape> {
    let mut merged = Shape {
        start: a.start.clone(), // We assume that the start is not in other
        curves: Vec::new(),
        color: a.color.clone(),
    };

    let mut closed = false;
    let mut i_main = 0;
    let mut is_a_main = true;
    while !closed {
        let (m_p0, m_cp0, m_cp1, m_p1) = if is_a_main {
            i_main = i_main % a.curves.len();
            a.get_coords_of_curve(i_main)
        } else {
            i_main = i_main % b.curves.len();
            b.get_coords_of_curve(i_main)
        };

        let max_len_other = if is_a_main {
            a.curves.len()
        } else {
            b.curves.len()
        };

        let mut has_done = false;
        for i_b in 0..max_len_other {
            let (b_p0, b_cp0, b_cp1, b_p1) = if is_a_main {
                b.get_coords_of_curve(i_b)
            } else {
                a.get_coords_of_curve(i_b)
            };

            let intersection_points = intersection(
                &m_p0.borrow(),
                &m_cp0.borrow(),
                &m_cp1.borrow(),
                &m_p1.borrow(),
                &b_p0.borrow(),
                &b_cp0.borrow(),
                &b_cp1.borrow(),
                &b_p1.borrow(),
            );

            if !intersection_points.is_empty() {
                let point = intersection_points[0];

                let (new_cp0, new_cp1, new_p1, _, _) = add_smooth_result(
                    &m_p0.borrow(),
                    &m_cp0.borrow(),
                    &m_cp1.borrow(),
                    &m_p1.borrow(),
                    point.t1,
                );

                merged.curves.push(Curve::new(
                    Rc::new(RefCell::new(new_cp0)),
                    Rc::new(RefCell::new(new_cp1)),
                    Rc::new(RefCell::new(new_p1)),
                ));

                let (_, _, _, new_cp0, new_cp1) = add_smooth_result(
                    &b_p0.borrow(),
                    &b_cp0.borrow(),
                    &b_cp1.borrow(),
                    &b_p1.borrow(),
                    point.t2,
                );

                merged.curves.push(Curve::new(
                    Rc::new(RefCell::new(new_cp0)),
                    Rc::new(RefCell::new(new_cp1)),
                    b_p1.clone(),
                ));
                is_a_main = !is_a_main;
                i_main = i_b + 1;
                has_done = true;
                break;
            }
        }

        if has_done {
            continue;
        }

        merged.curves.push(Curve::new(m_cp0, m_cp1, m_p1));
        i_main += 1;

        if *merged.start.borrow() == *merged.curves.last().unwrap().p1.borrow() {
            closed = true;
        }
    }

    Some(merged)
}

#[cfg(test)]
mod test {
    use common::{types::Coord, Rgba};

    use crate::{create_circle, shape_boolean::union, Vgc};

    #[test]
    fn when_merge_two_circle() {
        let mut vgc = Vgc::new(Rgba::new(255, 255, 255, 255));

        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.2, 0.2);
        create_circle(&mut vgc, Coord::new(0.2, 0.0), 0.2, 0.2);

        let s1 = vgc.get_shape(0).expect("Shape should exist");
        let s2 = vgc.get_shape(1).expect("Shape should exist");

        let merged = union(&s1, &s2).expect("Should merge");

        assert_eq!(*(merged.curves[1].p1.borrow()), Coord::new(0.2, 0.20001104));
        assert_eq!(merged.curves.len(), 8);
        assert_eq!(merged.to_path(),"M 0 0.20001104 C 0.03648475 0.19992407 0.07062003 0.19018893 0.1 0.17321144 C 0.12937993 0.19018891 0.16351523 0.19992408 0.2 0.20001104 C 0.3106854 0.19974719 0.3997472 0.110685386 0.40001106 0 C 0.3997472 -0.110685386 0.3106854 -0.19974719 0.2 -0.20001104 C 0.16351524 -0.19992407 0.12937997 -0.19018893 0.10000001 -0.17321144 C 0.07062003 -0.19018894 0.03648475 -0.19992407 0 -0.20001104 C -0.110685386 -0.19974719 -0.19974719 -0.110685386 -0.20001104 0 C -0.19974719 0.110685386 -0.110685386 0.19974719 0 0.20001104 Z");
    }

    #[test]
    fn when_merge_ovals_with_no_valid_p() {
        let vgc = crate::generate_from_push(vec![
            vec![
                Coord::new(0.0, 0.3),

                Coord::new(0.8, 0.3),
                Coord::new(0.8, -0.3),
                Coord::new(0.0, -0.3),

                Coord::new(-0.8, -0.3),
                Coord::new(-0.8, 0.3),
                Coord::new(0.0, 0.3),
            ],
            vec![
                Coord::new(0.3, 0.0),

                Coord::new(0.3, 0.8),
                Coord::new(-0.3, 0.8),
                Coord::new(-0.3, 0.0),

                Coord::new(-0.3, -0.8),
                Coord::new(0.3, -0.8),
                Coord::new(0.3, 0.0),
            ],
        ]);

        let s1 = vgc.get_shape(0).expect("Shape should exist");
        let s2 = vgc.get_shape(1).expect("Shape should exist");

        let merged = union(&s1, &s2).expect("Should merge");

        assert_eq!(merged.curves.len(), 4);
        println!("{}", merged.to_path());
    }
}
