use common::types::{Coord, Rect};
use polynomen::Poly;

use crate::curve::{add_smooth_result, cubic_bezier};

pub fn bounding_box(p0: &Coord, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Rect {
    let extremities = extremites(&p0, &cp0, &cp1, &p1);

    let mut min = Coord::new(f32::MAX, f32::MAX);
    let mut max = Coord::new(f32::MIN, f32::MIN);

    for t in extremities {
        let value = cubic_bezier(t, &p0, &cp0, &cp1, &p1);

        min = Coord::min(&min, &value);
        max = Coord::max(&max, &value);
    }

    Rect {
        top_left: min,
        bottom_right: max,
    }
}

/// Returns t extremities of the curve in the order of t smallest to t largest
pub fn extremites(p0: &Coord, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Vec<f32> {
    let mut vec = Vec::new();
    vec.push(0.0);
    vec.push(1.0);

    // for first derivative
    let d1a = 3.0 * (-p0 + 3.0 * cp0 - 3.0 * cp1 + p1);
    let d1b = 6.0 * (p0 - 2.0 * cp0 + cp1);
    let d1c = 3.0 * (cp0 - p0);

    // for second derivative
    let d2a = 6.0 * (-p0 + 3.0 * cp0 - 3.0 * cp1 + p1);
    let d2b = 6.0 * (p0 - 2.0 * cp0 + cp1);

    Poly::new_from_coeffs(&vec![d1c.x() / d1a.x(), d1b.x() / d1a.x(), 1.0])
        .real_roots()
        .and_then(|roots| {
            for root in roots {
                if root > 0.0 && root < 1.0 {
                    vec.push(root);
                }
            }
            Some(())
        });

    Poly::new_from_coeffs(&vec![d1c.y() / d1a.y(), d1b.y() / d1a.y(), 1.0])
        .real_roots()
        .and_then(|roots| {
            for root in roots {
                if root > 0.0 && root < 1.0 {
                    vec.push(root);
                }
            }
            Some(())
        });

    Poly::new_from_coeffs(&vec![d2b.x() / d2a.x(), 1.0])
        .real_roots()
        .and_then(|roots| {
            for root in roots {
                if root > 0.0 && root < 1.0 {
                    vec.push(root);
                }
            }
            Some(())
        });

    Poly::new_from_coeffs(&vec![d2b.y() / d2a.y(), 1.0])
        .real_roots()
        .and_then(|roots| {
            for root in roots {
                if root > 0.0 && root < 1.0 {
                    vec.push(root);
                }
            }
            Some(())
        });

    vec.sort_by(|a, b| a.partial_cmp(b).expect("No Nan value possible"));
    vec
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IntersectionPoint {
    /// The point of the overlapp
    pub coord: Coord,
    /// The t value of the first curve
    pub t1: f32,
    /// The t value of the second curve
    pub t2: f32,
}

struct IntersectionToDo {
    c1_p0: Coord,
    c1_cp0: Coord,
    c1_cp1: Coord,
    c1_p1: Coord,
    c2_p0: Coord,
    c2_cp0: Coord,
    c2_cp1: Coord,
    c2_p1: Coord,
    t1: f32,
    t2: f32,
    level: i32,
}

pub fn intersection(
    c1_p0: &Coord,
    c1_cp0: &Coord,
    c1_cp1: &Coord,
    c1_p1: &Coord,
    c2_p0: &Coord,
    c2_cp0: &Coord,
    c2_cp1: &Coord,
    c2_p1: &Coord,
) -> Vec<IntersectionPoint> {
    let mut todo = Vec::new();
    let first_todo = IntersectionToDo {
        c1_p0: *c1_p0,
        c1_cp0: *c1_cp0,
        c1_cp1: *c1_cp1,
        c1_p1: *c1_p1,
        c2_p0: *c2_p0,
        c2_cp0: *c2_cp0,
        c2_cp1: *c2_cp1,
        c2_p1: *c2_p1,
        t1: 0.5,
        t2: 0.5,
        level: 1,
    };
    todo.push(first_todo);

    run_intersection(&mut todo)
}

fn run_intersection(todo: &mut Vec<IntersectionToDo>) -> Vec<IntersectionPoint> {
    let mut res: Vec<IntersectionPoint> = Vec::new();
    while todo.len() > 0 {
        let cu = todo.pop().expect("No empty todo");
        let c1_rect = bounding_box(&cu.c1_p0, &cu.c1_cp0, &cu.c1_cp1, &cu.c1_p1);
        let c2_rect = bounding_box(&cu.c2_p0, &cu.c2_cp0, &cu.c2_cp1, &cu.c2_p1);

        if !own_intersect(&c1_rect, &c2_rect) {
            continue;
        }

        let max = Rect::max(&c1_rect, &c2_rect);

        if max.approx_diagonal() < f32::EPSILON * f32::EPSILON * 1.0 || cu.level > 35 {
            let rtn = IntersectionPoint {
                coord: cubic_bezier(cu.t1, &cu.c1_p0, &cu.c1_cp0, &cu.c1_cp1, &cu.c1_p1),
                t1: cu.t1,
                t2: cu.t2,
            };

            let mut is_present = false;
            for r in &res {
                if coord_equal(&rtn.coord, &r.coord) {
                    is_present = true;
                    break;
                }
            }

            if !is_present {
                res.push(rtn);
                // if cu.level > 30 {
                //     println!("Max level reached with rect {:#?}, approx diagonal: {}, width {}, height {}", max, max.approx_diagonal(), max.width(), max.height());
                //     println!("eps: {}", f32::EPSILON * f32::EPSILON);
                // }
            }

            continue;
        }

        let (c1_1_cp0, c1_1_cp1, c1_1_p1, c1_2_cp0, c1_2_cp1) =
            add_smooth_result(&cu.c1_p0, &cu.c1_cp0, &cu.c1_cp1, &cu.c1_p1, 0.5);
        let c1_1_p0 = cu.c1_p0;
        let c1_2_p0 = c1_1_p1;
        let c1_2_p1 = cu.c1_p1;

        let (c2_1_cp0, c2_1_cp1, c2_1_p1, c2_2_cp0, c2_2_cp1) =
            add_smooth_result(&cu.c2_p0, &cu.c2_cp0, &cu.c2_cp1, &cu.c2_p1, 0.5);
        let c2_1_p0 = cu.c2_p0;
        let c2_2_p0 = c2_1_p1;
        let c2_2_p1 = cu.c2_p1;

        let level = cu.level + 1;
        let t_change = 1.0 / (2.0f32).powi(level);

        let res_c1_1_c2_1 = IntersectionToDo {
            c1_p0: c1_1_p0,
            c1_cp0: c1_1_cp0,
            c1_cp1: c1_1_cp1,
            c1_p1: c1_1_p1,
            c2_p0: c2_1_p0,
            c2_cp0: c2_1_cp0,
            c2_cp1: c2_1_cp1,
            c2_p1: c2_1_p1,
            t1: cu.t1 - t_change,
            t2: cu.t2 - t_change,
            level,
        };
        todo.push(res_c1_1_c2_1);

        let res_c1_1_c2_2 = IntersectionToDo {
            c1_p0: c1_1_p0,
            c1_cp0: c1_1_cp0,
            c1_cp1: c1_1_cp1,
            c1_p1: c1_1_p1,
            c2_p0: c2_2_p0,
            c2_cp0: c2_2_cp0,
            c2_cp1: c2_2_cp1,
            c2_p1: c2_2_p1,
            t1: cu.t1 - t_change,
            t2: cu.t2 + t_change,
            level,
        };
        todo.push(res_c1_1_c2_2);

        let res_c1_2_c2_1 = IntersectionToDo {
            c1_p0: c1_2_p0,
            c1_cp0: c1_2_cp0,
            c1_cp1: c1_2_cp1,
            c1_p1: c1_2_p1,
            c2_p0: c2_1_p0,
            c2_cp0: c2_1_cp0,
            c2_cp1: c2_1_cp1,
            c2_p1: c2_1_p1,
            t1: cu.t1 + t_change,
            t2: cu.t2 - t_change,
            level,
        };
        todo.push(res_c1_2_c2_1);

        let res_c1_2_c2_2 = IntersectionToDo {
            c1_p0: c1_2_p0,
            c1_cp0: c1_2_cp0,
            c1_cp1: c1_2_cp1,
            c1_p1: c1_2_p1,
            c2_p0: c2_2_p0,
            c2_cp0: c2_2_cp0,
            c2_cp1: c2_2_cp1,
            c2_p1: c2_2_p1,
            t1: cu.t1 + t_change,
            t2: cu.t2 + t_change,
            level,
        };
        todo.push(res_c1_2_c2_2);
    }

    res
}

fn coord_equal(a: &Coord, b: &Coord) -> bool {
    f32::abs(a.x() - b.x()) <= f32::EPSILON * 1.0 && f32::abs(a.y() - b.y()) <= f32::EPSILON * 1.0
}

//Intersection between two rectangles
// true if inside each other
// true if top left equal
// false if bottom right equal
fn own_intersect(a: &Rect, b: &Rect) -> bool {
    a.top_left.x() <= b.bottom_right.x()
        && a.bottom_right.x() > b.top_left.x()
        && a.top_left.y() <= b.bottom_right.y()
        && a.bottom_right.y() > b.top_left.y()
}

#[cfg(test)]
mod tests {
    use common::pures::{Affine, Vec2};
    use float_cmp::assert_approx_eq;

    use super::super::curve::cubic_bezier;
    use super::*;

    #[test]
    fn test_bounding_box() {
        let p0 = Coord::new(110.0, 150.0);
        let cp0 = Coord::new(25.0, 190.0);
        let cp1 = Coord::new(210.0, 250.0);
        let p1 = Coord::new(210.0, 30.0);

        let rect = bounding_box(&p0, &cp0, &cp1, &p1);

        assert_approx_eq!(f32, rect.top_left.x(), 87.6645332689);
        assert_approx_eq!(f32, rect.top_left.y(), 30.0);
        assert_approx_eq!(f32, rect.bottom_right.x(), 210.0);
        assert_approx_eq!(f32, rect.bottom_right.y(), 188.862345822);
    }

    #[test]
    fn test_extremites() {
        let p0 = Coord::new(110.0, 150.0);
        let cp0 = Coord::new(25.0, 190.0);
        let cp1 = Coord::new(210.0, 250.0);
        let p1 = Coord::new(210.0, 30.0);

        let vec = extremites(&p0, &cp0, &cp1, &p1);

        assert_eq!(vec.len(), 6);
        assert_approx_eq!(f32, vec[0], 0.0);
        assert_approx_eq!(f32, vec[1], 0.066666666667);
        assert_approx_eq!(f32, vec[2], 0.186813186813);
        assert_approx_eq!(f32, vec[3], 0.437850957522);
        assert_approx_eq!(f32, vec[4], 0.593406593407);
        assert_approx_eq!(f32, vec[5], 1.0);
    }

    #[test]
    fn when_two_perpendicular_lines_then_intersection() {
        let c1_p0 = Coord::new(0.0, -1.0);
        let c1_cp0 = Coord::new(0.0, -1.0);
        let c1_cp1 = Coord::new(0.0, 1.0);
        let c1_p1 = Coord::new(0.0, 1.0);

        let c2_p0 = Coord::new(-1.0, 0.0);
        let c2_cp0 = Coord::new(-1.0, 0.0);
        let c2_cp1 = Coord::new(1.0, 0.0);
        let c2_p1 = Coord::new(1.0, 0.0);

        let res = intersection(
            &c1_p0, &c1_cp0, &c1_cp1, &c1_p1, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1,
        );

        assert_eq!(res.len(), 1);
        assert_approx_eq!(Coord, res[0].coord, Coord::new(0.0, 0.0));
        assert_approx_eq!(f32, res[0].t1, 0.5);
        assert_approx_eq!(f32, res[0].t2, 0.5);

        assert_approx_eq!(
            Coord,
            cubic_bezier(res[0].t2, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1),
            res[0].coord
        );
        assert_approx_eq!(
            Coord,
            cubic_bezier(res[0].t1, &c1_p0, &c1_cp0, &c1_cp1, &c1_p1),
            res[0].coord
        );
    }

    #[test]
    fn when_two_complex_curves_then_intersection() {
        let m = Affine::identity()
            .translate(Vec2::new(-20.0, -20.0))
            .scale(Vec2::new(1.0 / (235.0 - 20.0), 1.0 / (235.0 - 20.0)));

        let c1_p0 = Coord::new(50.0, 35.0).transform(&m);
        let c1_cp0 = Coord::new(45.0, 235.0).transform(&m);
        let c1_cp1 = Coord::new(220.0, 235.0).transform(&m);
        let c1_p1 = Coord::new(220.0, 135.0).transform(&m);

        let c2_p0 = Coord::new(20.0, 150.0).transform(&m);
        let c2_cp0 = Coord::new(120.0, 20.0).transform(&m);
        let c2_cp1 = Coord::new(220.0, 95.0).transform(&m);
        let c2_p1 = Coord::new(140.0, 240.0).transform(&m);

        let res = intersection(
            &c1_p0, &c1_cp0, &c1_cp1, &c1_p1, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1,
        );

        assert_eq!(res.len(), 2);
        assert!(res[0].t1 != res[1].t1);

        assert_approx_eq!(
            Coord,
            cubic_bezier(res[0].t1, &c1_p0, &c1_cp0, &c1_cp1, &c1_p1),
            res[0].coord
        );
        assert_approx_eq!(
            Coord,
            cubic_bezier(res[0].t2, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1),
            res[0].coord
        );

        assert_approx_eq!(
            Coord,
            cubic_bezier(res[1].t1, &c1_p0, &c1_cp0, &c1_cp1, &c1_p1),
            res[1].coord
        );
        assert_approx_eq!(
            Coord,
            cubic_bezier(res[1].t2, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1),
            res[1].coord
        );
    }

    #[test]
    fn when_two_complex_curves_then_intersection_3() {
        let m = Affine::identity()
            .translate(Vec2::new(-20.0, -20.0))
            .scale(Vec2::new(1.0 / (235.0 - 20.0), 1.0 / (235.0 - 20.0)));

        let c1_p0 = Coord::new(50.0, 35.0).transform(&m);
        let c1_cp0 = Coord::new(45.0, 235.0).transform(&m);
        let c1_cp1 = Coord::new(220.0, 235.0).transform(&m);
        let c1_p1 = Coord::new(220.0, 135.0).transform(&m);

        let c2_p0 = Coord::new(81.0, 113.0).transform(&m);
        let c2_cp0 = Coord::new(20.0, 208.0).transform(&m);
        let c2_cp1 = Coord::new(220.0, 95.0).transform(&m);
        let c2_p1 = Coord::new(140.0, 240.0).transform(&m);

        let res = intersection(
            &c1_p0, &c1_cp0, &c1_cp1, &c1_p1, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1,
        );

        assert_eq!(res.len(), 3);
        assert!(res[0].t1 != res[1].t1 && res[0].t1 != res[2].t1);

        assert_approx_eq!(
            Coord,
            cubic_bezier(res[0].t1, &c1_p0, &c1_cp0, &c1_cp1, &c1_p1),
            res[0].coord
        );
        assert_approx_eq!(
            Coord,
            cubic_bezier(res[0].t2, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1),
            res[0].coord
        );

        assert_approx_eq!(
            Coord,
            cubic_bezier(res[1].t1, &c1_p0, &c1_cp0, &c1_cp1, &c1_p1),
            res[1].coord
        );
        assert_approx_eq!(
            Coord,
            cubic_bezier(res[1].t2, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1),
            res[1].coord
        );

        assert_approx_eq!(
            Coord,
            cubic_bezier(res[2].t1, &c1_p0, &c1_cp0, &c1_cp1, &c1_p1),
            res[2].coord
        );
        assert_approx_eq!(
            Coord,
            cubic_bezier(res[2].t2, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1),
            res[2].coord
        );
    }
}
