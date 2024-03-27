/*
Implementation of boolean operations on shapes.
For Shape A and B
Union : A OR B
Intersection : A AND B
Difference : A NOR B
Symmetric Difference : A XOR B
*/

use std::{cell::RefCell, rc::Rc};

use common::{pures::Vec2, types::Coord};

use crate::{
    coord::CoordPtr,
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

// When calculating the union of two shapes, we need to find all the intersection points between the two shapes.
// GreinerShape is a representation of a shape where all intersection points are added as separate coordinates and marked as such.
#[derive(Default)]
struct GreinerShape {
    pub coords: Vec<GreinerCoord>,
}

impl GreinerShape {
    pub fn insert_intersection(
        &mut self,
        shape_index: usize,
        cp0: Coord,
        cp1l: Coord,
        p1: Coord,
        cp1r: Coord,
        cp2: Coord,
        other_shape_index: usize,
    ) {
        self.coords[shape_index + 1].coord = cp0;
        self.coords[shape_index + 1].coord_ptr = None;

        self.coords[shape_index + 2].coord = cp1l;
        self.coords[shape_index + 2].coord_ptr = None;

        self.coords.insert(
            shape_index + 3,
            GreinerCoord::new_intersect(p1, other_shape_index + 3),
        );
        self.coords
            .insert(shape_index + 4, GreinerCoord::new2(cp1r));
        self.coords.insert(shape_index + 5, GreinerCoord::new2(cp2));
        let a = 2;
    }
}

#[derive(Default)]
struct GreinerCoord {
    pub coord: Coord,
    pub intersect: bool,
    pub is_entry: bool, // Or exit
    pub other_shape_index: Option<usize>,
    pub coord_ptr: Option<CoordPtr>,
}

impl GreinerCoord {
    pub fn new(coord_ptr: &CoordPtr) -> Self {
        let coord = *coord_ptr.borrow();
        Self {
            coord,
            intersect: false,
            is_entry: false,
            other_shape_index: None,
            coord_ptr: Some(coord_ptr.clone()),
        }
    }

    pub fn new2(coord: Coord) -> Self {
        Self {
            coord,
            intersect: false,
            is_entry: false,
            other_shape_index: None,
            coord_ptr: None,
        }
    }

    pub fn new_intersect(coord: Coord, other_shape_index: usize) -> Self {
        Self {
            coord,
            intersect: true,
            is_entry: false,
            other_shape_index: Some(other_shape_index),
            coord_ptr: None,
        }
    }
}

#[allow(dead_code)]
fn find_all_intersecion(a: &Shape, b: &Shape) -> (GreinerShape, GreinerShape) {
    let mut a_greiner = GreinerShape::default();

    a_greiner.coords.push(GreinerCoord::new(&a.start));
    for i in 0..a.curves.len() {
        let (_, a_cp0, a_cp1, a_p1) = a.get_coords_of_curve(i);
        a_greiner.coords.push(GreinerCoord::new(&a_cp0));
        a_greiner.coords.push(GreinerCoord::new(&a_cp1));
        a_greiner.coords.push(GreinerCoord::new(&a_p1));
    }

    let mut b_greiner = GreinerShape::default();

    b_greiner.coords.push(GreinerCoord::new(&b.start));
    for i in 0..b.curves.len() {
        let (_, b_cp0, b_cp1, b_p1) = b.get_coords_of_curve(i);
        b_greiner.coords.push(GreinerCoord::new(&b_cp0));
        b_greiner.coords.push(GreinerCoord::new(&b_cp1));
        b_greiner.coords.push(GreinerCoord::new(&b_p1));
    }

    for mut i in (0..a_greiner.coords.len() - 1).step_by(3) {
        let a_p0 = a_greiner.coords[i].coord.clone();
        let a_cp0 = &a_greiner.coords[i + 1].coord.clone();
        let a_cp1 = &a_greiner.coords[i + 2].coord.clone();
        let a_p1 = &a_greiner.coords[i + 3].coord.clone();

        for mut j in (0..a_greiner.coords.len() - 1).step_by(3) {
            let b_p0 = &b_greiner.coords[j];
            let b_cp0 = &b_greiner.coords[j + 1];
            let b_cp1 = &b_greiner.coords[j + 2];
            let b_p1 = &b_greiner.coords[j + 3];

            let intersection_points = intersection(
                &a_p0,
                &a_cp0,
                &a_cp1,
                &a_p1,
                &b_p0.coord,
                &b_cp0.coord,
                &b_cp1.coord,
                &b_p1.coord,
            );

            match intersection_points.len() {
                0 => {
                    // No intersection
                    continue;
                }
                1 => {
                    let eps = 2.0 * f32::EPSILON;
                    if (intersection_points[0].t1 <= eps || intersection_points[0].t1 >= 1.0)
                        || (intersection_points[0].t2 <= eps || intersection_points[0].t2 >= 1.0)
                    {
                        // Intersection at the start or end of the curve
                        continue;
                    } 
                    let (a_new_cp0, a_new_cp11, a_new_p1, a_new_cp1r, a_new_cp2) =
                        add_smooth_result(
                            &a_p0,
                            &a_cp0,
                            &a_cp1,
                            &a_p1,
                            intersection_points[0].t1,
                        );

                    let (b_new_cp0, b_new_cp11, b_new_p1, b_new_cp1r, b_new_cp2) =
                        add_smooth_result(
                            &b_p0.coord,
                            &b_cp0.coord,
                            &b_cp1.coord,
                            &b_p1.coord,
                            intersection_points[0].t2,
                        );
                    b_greiner.insert_intersection(
                        j, b_new_cp0, b_new_cp11, b_new_p1, b_new_cp1r, b_new_cp2, i,
                    );

                    a_greiner.insert_intersection(
                        i, a_new_cp0, a_new_cp11, a_new_p1, a_new_cp1r, a_new_cp2, j,
                    );
                }
                _ => {
                    todo!("Handle multiple intersection points")
                }
            }
        }
    }

    //Remove the last point wich is a duplicate of the first
    a_greiner.coords.pop();
    b_greiner.coords.pop();

    (a_greiner, b_greiner)
}

#[cfg(test)]
mod test {
    use common::{types::Coord, Rgba};

    use crate::{
        create_circle,
        shape_boolean::{find_all_intersecion, union},
        Vgc,
    };

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
    fn when_merge_circle2() {
        let mut vgc = Vgc::new(Rgba::new(255, 255, 255, 255));

        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.2, 0.2);
        create_circle(&mut vgc, Coord::new(0.2, 0.0), 0.2, 0.2);

        let s1 = vgc.get_shape(0).expect("Shape should exist");
        let s2 = vgc.get_shape(1).expect("Shape should exist");

        let (a, b) = find_all_intersecion(s1, s2);

        assert_eq!(a.coords.len(), 18);
        assert_eq!(b.coords.len(), 18);
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
