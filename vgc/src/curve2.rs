use std::vec;

use common::{
    dbg_str,
    types::{Coord, Rect},
};
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
    match curves_overlap(c1_p0, c1_cp0, c1_cp1, c1_p1, c2_p0, c2_cp0, c2_cp1, c2_p1) {
        Overlap::None => {}
        _ => return Vec::new(),
    }

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

    let res = match run_intersection(&mut todo) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{}", dbg_str!("{}", e));
            log::debug!("c1: {:?} {:?} {:?} {:?}", c1_p0, c1_cp0, c1_cp1, c1_p1);
            log::debug!("c2: {:?} {:?} {:?} {:?}", c2_p0, c2_cp0, c2_cp1, c2_p1);

            return Vec::new();
        }
    };

    //Remove the intersection at the extremities
    let mut t_res = Vec::new();
    for r in res {
        let t1_t2_0 = coord_equal(&r.coord, c1_p0) && coord_equal(&r.coord, c2_p0);
        let t1_0_t2_1 = coord_equal(&r.coord, c1_p0) && coord_equal(&r.coord, c2_p1);
        let t1_1_t2_0 = coord_equal(&r.coord, c1_p1) && coord_equal(&r.coord, c2_p0);
        let t1_t2_1 = coord_equal(&r.coord, c1_p1) && coord_equal(&r.coord, c2_p1);

        if t1_t2_0 || t1_0_t2_1 || t1_1_t2_0 || t1_t2_1 {
            continue;
        }

        t_res.push(r);
    }

    t_res
}

const PRECISION: f32 = 4.0;

fn run_intersection(todo: &mut Vec<IntersectionToDo>) -> Result<Vec<IntersectionPoint>, String> {
    let mut res: Vec<IntersectionPoint> = Vec::new();
    let mut max_todo = 0;
    let mut i = 0;
    while todo.len() > 0 {
        max_todo = max_todo.max(todo.len());
        //println!("i: {} todo: {}", i, todo.len());
        if i > 2000 {
            return Err("Max iteration reached, stopping the intersection calculation. We may be in an infinite loop because of overlapping curves.".to_string());
        }
        i += 1;

        let cu = todo.pop().expect("No empty todo");
        let c1_rect = bounding_box(&cu.c1_p0, &cu.c1_cp0, &cu.c1_cp1, &cu.c1_p1);
        let c2_rect = bounding_box(&cu.c2_p0, &cu.c2_cp0, &cu.c2_cp1, &cu.c2_p1);

        if !own_intersect(&c1_rect, &c2_rect) {
            continue;
        }

        let max = Rect::max(&c1_rect, &c2_rect);

        let max_iter = 30; //30 and 0.5 are over kill for precision, but it's better to have too much than not enough
        let min_diago = f32::EPSILON * PRECISION * f32::EPSILON * PRECISION * 0.5;
        if max.approx_diagonal() < min_diago || cu.level > max_iter {
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
                if cu.level > max_iter {
                    println!("Max level reached with rect {:#?},\napprox diagonal: {}, width {}, height {}", max, max.approx_diagonal(), max.width(), max.height());
                    println!("eps            : {}", min_diago);
                }
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

    Ok(res)
}

pub fn coord_equal(a: &Coord, b: &Coord) -> bool {
    f32::abs(a.x() - b.x()) <= f32::EPSILON * PRECISION
        && f32::abs(a.y() - b.y()) <= f32::EPSILON * PRECISION
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

pub fn intersection_with_y(p0: &Coord, cp0: &Coord, cp1: &Coord, p1: &Coord, y: f32) -> Vec<f32> {
    let mut vec = Vec::new();

    let p0y = p0.y() as f64;
    let cp0y = cp0.y() as f64;
    let cp1y = cp1.y() as f64;
    let p1y = p1.y() as f64;
    let y = y as f64;

    let coeff0: f64 = p0y - y;
    let coeff1: f64 = 3.0 * (cp0y - p0y);
    let coeff2: f64 = 3.0 * (p0y - 2.0 * cp0y + cp1y);
    let coeff3: f64 = -p0y + 3.0 * cp0y - 3.0 * cp1y + p1y;

    //We got a line parallel to y
    if f64::abs(coeff3) < f64::EPSILON
        && f64::abs(coeff2) < f64::EPSILON
        && f64::abs(coeff1) < f64::EPSILON
    {
        return vec;
    }

    let poly = Poly::new_from_coeffs(&vec![coeff0, coeff1, coeff2, coeff3]);

    let croots = poly.complex_roots();

    for root in croots {
        //For the even-odd rule, we don't care if root is at 0.0 or 1.0, because it need to add 2 intersections
        if 0.0 < root.0 && root.0 < 1.0 && f64::abs(root.1) < (f32::EPSILON as f64) {
            vec.push(root.0 as f32);
        }
    }

    vec
}

pub enum Overlap {
    ASmallerAndInsideB,
    BSmallerAndInsideA,
    None,
}
pub fn curves_overlap(
    c1_p0: &Coord,
    c1_cp0: &Coord,
    c1_cp1: &Coord,
    c1_p1: &Coord,
    c2_p0: &Coord,
    c2_cp0: &Coord,
    c2_cp1: &Coord,
    c2_p1: &Coord,
) -> Overlap {
    let ts = vec![
        0.006263, 0.108011, 0.278309, 0.548986, 0.85558, 0.935159, 0.977084,
    ];
    let end_value = ts[ts.len() - 1];
    for t in &ts {
        let c1 = cubic_bezier(*t, c1_p0, c1_cp0, c1_cp1, c1_p1);
        let intesections = intersection_with_y(c2_p0, c2_cp0, c2_cp1, c2_p1, c1.y());

        let mut one_equal = false;
        for t2 in intesections {
            let c2 = cubic_bezier(t2, c2_p0, c2_cp0, c2_cp1, c2_p1);
            if f32::abs(c1.x() - c2.x()) < f32::EPSILON * PRECISION {
                one_equal = true;
                break;
            }
        }
        if !one_equal {
            break;
        }

        //If we reach the end, it means that all the x are equal
        if *t == end_value {
            return Overlap::ASmallerAndInsideB;
        }
    }

    for t in ts {
        let c2 = cubic_bezier(t, c2_p0, c2_cp0, c2_cp1, c2_p1);
        let intesections = intersection_with_y(c1_p0, c1_cp0, c1_cp1, c1_p1, c2.y());

        let mut one_equal = false;
        for t2 in intesections {
            let c1 = cubic_bezier(t2, c1_p0, c1_cp0, c1_cp1, c1_p1);
            if f32::abs(c1.x() - c2.x()) < f32::EPSILON * PRECISION {
                one_equal = true;
                break;
            }
        }
        if !one_equal {
            break;
        }

        //If we reach the end, it means that all the x are equal
        if t == end_value {
            return Overlap::BSmallerAndInsideA;
        }
    }

    return Overlap::None;
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

    #[test]
    fn given_complex_curve_when_intersect_with_y_then_3_t_found() {
        let p0 = Coord::new(52.0, 77.0);
        let cp0 = Coord::new(83.0, 249.0);
        let cp1 = Coord::new(133.0, 19.0);
        let p1 = Coord::new(172.0, 192.0);

        let vec = intersection_with_y(&p0, &cp0, &cp1, &p1, 137.0);

        assert_eq!(vec.len(), 3);
        assert_approx_eq!(f32, cubic_bezier(vec[0], &p0, &cp0, &cp1, &p1).y(), 137.0);
        assert_approx_eq!(f32, cubic_bezier(vec[1], &p0, &cp0, &cp1, &p1).y(), 137.0);
        assert_approx_eq!(f32, cubic_bezier(vec[2], &p0, &cp0, &cp1, &p1).y(), 137.0);
    }

    #[test]
    fn given_not_overlap_curves_when_overlap_none() {
        let p0 = Coord::new(0.0, 0.0);
        let cp0 = Coord::new(0.0, 0.0);
        let cp1 = Coord::new(0.0, 1.0);
        let p1 = Coord::new(0.0, 1.0);

        let p2 = Coord::new(1.0, 0.0);
        let cp2 = Coord::new(1.0, 0.0);
        let cp3 = Coord::new(1.0, 1.0);
        let p3 = Coord::new(1.0, 1.0);

        let res = curves_overlap(&p0, &cp0, &cp1, &p1, &p2, &cp2, &cp3, &p3);

        assert!(matches!(res, Overlap::None));
    }

    #[test]
    fn given_same_curve_when_overlap_a_smaller_and_inside_b() {
        let p0 = Coord::new(0.0, 0.0);
        let cp0 = Coord::new(0.0, 0.0);
        let cp1 = Coord::new(1.0, 1.0);
        let p1 = Coord::new(1.0, 1.0);

        let p2 = Coord::new(0.0, 0.0);
        let cp2 = Coord::new(0.0, 0.0);
        let cp3 = Coord::new(1.0, 1.0);
        let p3 = Coord::new(1.0, 1.0);

        let res = curves_overlap(&p0, &cp0, &cp1, &p1, &p2, &cp2, &cp3, &p3);

        assert!(matches!(res, Overlap::ASmallerAndInsideB));
    }

    #[test]
    fn given_overlap_curvewhen_overlap_a_smaller_and_inside_b() {
        let c1_p0 = Coord::new(0.0, 0.0);
        let c1_cp0 = Coord::new(0.1, 0.4);
        let c1_cp1 = Coord::new(0.5, 1.0);
        let c1_p1 = Coord::new(1.0, 1.0);

        let (c2_cp0, c2_cp1, c2_p1, _, _) =
            add_smooth_result(&c1_p0, &c1_cp0, &c1_cp1, &c1_p1, 0.6788);

        let res = curves_overlap(
            &c1_p0, &c1_cp0, &c1_cp1, &c1_p1, &c2_p1, &c2_cp1, &c2_cp0, &c1_p0,
        );

        assert!(matches!(res, Overlap::BSmallerAndInsideA));
    }

    #[test]
    fn given_curves_when_intersection_then_max_iteration() {
        // c1: Coord { c: Vec2 { x: 1.0, y: -1.0 } } Coord { c: Vec2 { x: 1.0, y: -1.0 } } Coord { c: Vec2 { x: 1.0, y: 1.0 } } Coord { c: Vec2 { x: 1.0, y: 1.0 } } main.js:4311:13
        //c2: Coord { c: Vec2 { x: 1.0, y: 1.0 } } Coord { c: Vec2 { x: 1.0, y: 1.0 } } Coord { c: Vec2 { x: 0.0, y: 0.0 } } Coord { c: Vec2 { x: 0.0, y: 0.0 } }

        let c1_p0 = Coord::new(1.0, -1.0);
        let c1_cp0 = Coord::new(1.0, -1.0);
        let c1_cp1 = Coord::new(1.0, 1.0);
        let c1_p1 = Coord::new(1.0, 1.0);

        let c2_p0 = Coord::new(1.0, 1.0);
        let c2_cp0 = Coord::new(1.0, 1.0);
        let c2_cp1 = Coord::new(0.0, 0.0);
        let c2_p1 = Coord::new(0.0, 0.0);

        let res = intersection(
            &c1_p0, &c1_cp0, &c1_cp1, &c1_p1, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1,
        );

        assert_eq!(res.len(), 0); //gets filtered out
    }
}
