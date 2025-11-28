use common::pures::Vec2;
use common::{
    dbg_str,
    types::{Coord, Rect},
    PRECISION,
};
use roots::{find_roots_cubic, find_roots_linear, find_roots_quadratic};

use crate::math::curve::{add_smooth_result, cubic_bezier};

use super::line_intersection::line_intersection;

#[allow(dead_code)]
pub fn bounding_box(p0: &Coord, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Rect {
    let extremities = extremites(p0, cp0, cp1, p1);

    let mut min = Coord::new(f32::MAX, f32::MAX);
    let mut max = Coord::new(f32::MIN, f32::MIN);

    for t in extremities {
        let value = cubic_bezier(t, p0, cp0, cp1, p1);

        min = Coord::min(&min, &value);
        max = Coord::max(&max, &value);
    }

    Rect {
        top_left: min,
        bottom_right: max,
    }
}

/// Returns the bounding box of the curve without calculating the extremities
/// This is faster and more stable than `bounding_box` but less precise
pub fn quick_bounding_box(p0: &Coord, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Rect {
    let mut min = Coord::new(f32::MAX, f32::MAX);
    let mut max = Coord::new(f32::MIN, f32::MIN);

    for c in &[p0, cp0, cp1, p1] {
        min = Coord::min(&min, c);
        max = Coord::max(&max, c);
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

    let p0x = p0.x() as f64;
    let p0y = p0.y() as f64;
    let cp0x = cp0.x() as f64;
    let cp0y = cp0.y() as f64;
    let cp1x = cp1.x() as f64;
    let cp1y = cp1.y() as f64;
    let p1x = p1.x() as f64;
    let p1y = p1.y() as f64;

    // for first derivative
    let d1ax = 3.0 * (-p0x + 3.0 * cp0x - 3.0 * cp1x + p1x);
    let d1ay = 3.0 * (-p0y + 3.0 * cp0y - 3.0 * cp1y + p1y);
    let d1bx = 6.0 * (p0x - 2.0 * cp0x + cp1x);
    let d1by = 6.0 * (p0y - 2.0 * cp0y + cp1y);
    let d1cx = 3.0 * (cp0x - p0x);
    let d1cy = 3.0 * (cp0y - p0y);

    // For quadratic d1ax*t^2 + d1bx*t + d1cx = 0
    // find_roots_quadratic takes coefficients (a, b, c) for at^2 + bt + c = 0
    if d1ax.abs() > f64::EPSILON {
        let roots_result = find_roots_quadratic(d1ax, d1bx, d1cx);
        for root in roots_result.as_ref().iter() {
            if *root > 0.0 && *root < 1.0 {
                vec.push(*root);
            }
        }
    }

    if d1ay.abs() > f64::EPSILON {
        let roots_result = find_roots_quadratic(d1ay, d1by, d1cy);
        for root in roots_result.as_ref().iter() {
            if *root > 0.0 && *root < 1.0 {
                vec.push(*root);
            }
        }
    }

    // for second derivative
    let d2ax = 6.0 * (-p0x + 3.0 * cp0x - 3.0 * cp1x + p1x);
    let d2ay = 6.0 * (-p0y + 3.0 * cp0y - 3.0 * cp1y + p1y);
    let d2bx = 6.0 * (p0x - 2.0 * cp0x + cp1x);
    let d2by = 6.0 * (p0y - 2.0 * cp0y + cp1y);

    // For linear d2ax*t + d2bx = 0
    // find_roots_linear takes coefficients (a, b) for at + b = 0
    if d2ax.abs() > f64::EPSILON {
        let roots_result = find_roots_linear(d2ax, d2bx);
        for root in roots_result.as_ref().iter() {
            if *root > 0.0 && *root < 1.0 {
                vec.push(*root);
            }
        }
    }

    if d2ay.abs() > f64::EPSILON {
        let roots_result = find_roots_linear(d2ay, d2by);
        for root in roots_result.as_ref().iter() {
            if *root > 0.0 && *root < 1.0 {
                vec.push(*root);
            }
        }
    }

    vec.sort_by(|a, b| a.partial_cmp(b).expect("No Nan value possible"));
    vec.into_iter().map(|x| x as f32).collect()
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

#[derive(Debug)]
pub enum IntersectionResult {
    ASmallerAndInsideB,
    BSmallerAndInsideA,
    Pts(Vec<IntersectionPoint>),
    None,
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
) -> IntersectionResult {
    let rect1 = quick_bounding_box(c1_p0, c1_cp0, c1_cp1, c1_p1);
    let rect2 = quick_bounding_box(c2_p0, c2_cp0, c2_cp1, c2_p1);

    if !own_intersect(&rect1, &rect2) {
        return IntersectionResult::None;
    }

    match curves_overlap(c1_p0, c1_cp0, c1_cp1, c1_p1, c2_p0, c2_cp0, c2_cp1, c2_p1) {
        Overlap::ASmallerAndInsideB => return IntersectionResult::ASmallerAndInsideB,
        Overlap::BSmallerAndInsideA => return IntersectionResult::BSmallerAndInsideA,
        _ => {}
    }

    if (c1_p0 == c1_cp0 && c1_cp1 == c1_p1) && (c2_p0 == c2_cp0 && c2_cp1 == c2_p1) {
        return line_intersection(c1_p0, c2_p0, c2_p1, c1_p1);
    }

    let res = intersection_simple(c1_p0, c1_cp0, c1_cp1, c1_p1, c2_p0, c2_cp0, c2_cp1, c2_p1);
    if res.is_empty() {
        return IntersectionResult::None;
    }
    IntersectionResult::Pts(res)
}

fn intersection_simple(
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

    let mut res = match intersection_recsv(&mut todo) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{}", dbg_str!("{}", e));
            log::debug!("c1: {:?} {:?} {:?} {:?}", c1_p0, c1_cp0, c1_cp1, c1_p1);
            log::debug!("c2: {:?} {:?} {:?} {:?}", c2_p0, c2_cp0, c2_cp1, c2_p1);

            return Vec::new();
        }
    };
    //Set the t value to 0.0 or 1.0 if the intersection is at the extremities
    for r in &mut res {
        if &r.coord == c1_p0 {
            r.t1 = 0.0;
        }

        if &r.coord == c1_p1 {
            r.t1 = 1.0;
        }

        if &r.coord == c2_p0 {
            r.t2 = 0.0;
        }

        if &r.coord == c2_p1 {
            r.t2 = 1.0;
        }
    }
    res
}

fn intersection_recsv(todo: &mut Vec<IntersectionToDo>) -> Result<Vec<IntersectionPoint>, String> {
    let mut res: Vec<IntersectionPoint> = Vec::new();

    if todo[0].c1_p0 == todo[0].c2_p0 {
        res.push(IntersectionPoint {
            coord: todo[0].c1_p0,
            t1: 0.0,
            t2: 0.0,
        });
    }
    if todo[0].c1_p1 == todo[0].c2_p1 {
        res.push(IntersectionPoint {
            coord: todo[0].c1_p1,
            t1: 1.0,
            t2: 1.0,
        });
    }
    if todo[0].c1_p0 == todo[0].c2_p1 {
        res.push(IntersectionPoint {
            coord: todo[0].c1_p0,
            t1: 0.0,
            t2: 1.0,
        });
    }
    if todo[0].c1_p1 == todo[0].c2_p0 {
        res.push(IntersectionPoint {
            coord: todo[0].c1_p1,
            t1: 1.0,
            t2: 0.0,
        });
    }

    let mut max_todo = 0;
    let mut i = 0;
    while !todo.is_empty() {
        max_todo = max_todo.max(todo.len());
        // println!("i: {} todo: {}", i, todo.len());
        if i > 50_000 {
            return Err("Max iteration reached, stopping the intersection calculation. We may be in an infinite loop because of overlapping curves.".to_string());
        }
        i += 1;

        let cu = todo.pop().expect("No empty todo");
        let c1_rect = quick_bounding_box(&cu.c1_p0, &cu.c1_cp0, &cu.c1_cp1, &cu.c1_p1);
        let c2_rect = quick_bounding_box(&cu.c2_p0, &cu.c2_cp0, &cu.c2_cp1, &cu.c2_p1);

        if !own_intersect(&c1_rect, &c2_rect) {
            continue;
        }

        let max = Rect::max(&c1_rect, &c2_rect);

        let max_iter = 30; //30 and 0.5 are over kill for precision, but it's better to have too much than not enough
        let min_diago = PRECISION * PRECISION * 0.5;
        if max.approx_diagonal() < min_diago || cu.level > max_iter {
            let rtn = IntersectionPoint {
                coord: cubic_bezier(cu.t1, &cu.c1_p0, &cu.c1_cp0, &cu.c1_cp1, &cu.c1_p1),
                t1: cu.t1,
                t2: cu.t2,
            };

            let mut is_present = false;
            for r in &res {
                if rtn.coord == r.coord {
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

//Intersection between two rectangles
// true if inside each other
// true if top left equal
// false if bottom right equal
fn own_intersect(a: &Rect, b: &Rect) -> bool {
    a.top_left.x() <= b.bottom_right.x()
        && a.bottom_right.x() >= b.top_left.x()
        && a.top_left.y() <= b.bottom_right.y()
        && a.bottom_right.y() >= b.top_left.y()
}

pub fn intersection_with_y(p0: &Coord, cp0: &Coord, cp1: &Coord, p1: &Coord, y: f32) -> Vec<f32> {
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
        if p0y == y && p1y == y {
            return vec![0.0, 1.0];
        }
        return Vec::new();
    }

    // find_roots_cubic takes coefficients (a, b, c, d) for at^3 + bt^2 + ct + d = 0
    let roots_result = find_roots_cubic(coeff3, coeff2, coeff1, coeff0);

    let mut vec = Vec::new();
    for root in roots_result.as_ref().iter() {
        //For the even-odd rule, we don't care if root is at 0.0 or 1.0, because it need to add 2 intersections
        if 0.0 < (*root as f32) && (*root as f32) < 1.0 {
            if vec
                .iter()
                .any(|&x| f32::abs(x - *root as f32) < f32::EPSILON)
            {
                continue;
            }
            vec.push(*root as f32);
        }
    }
    //remove same value in vec

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
        0.006263, 0.108011, 0.278309, 0.347826, 0.406593, 0.437851, 0.548986, 0.686813, 0.85558,
        0.935159, 0.977084,
    ];

    let end_value = ts[ts.len() - 1];
    for t in &ts {
        let c1 = cubic_bezier(*t, c1_p0, c1_cp0, c1_cp1, c1_p1);
        let intesections = intersection_with_y(c2_p0, c2_cp0, c2_cp1, c2_p1, c1.y());

        let mut one_equal = false;
        for t2 in intesections {
            let c2 = cubic_bezier(t2, c2_p0, c2_cp0, c2_cp1, c2_p1);
            if c1 == c2 {
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

            if c1 == c2 {
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

    Overlap::None
}

#[cfg(test)]
mod tests {
    use common::pures::{Affine, Vec2};
    use common::types::{ScreenCoord, ScreenLength2d};
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

        let res = intersection_simple(
            &c1_p0, &c1_cp0, &c1_cp1, &c1_p1, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1,
        );

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].coord, Coord::new(0.0, 0.0));
        assert_approx_eq!(f32, res[0].t1, 0.5, epsilon = 0.000003);
        assert_approx_eq!(f32, res[0].t2, 0.5, epsilon = 0.000003);

        assert_eq!(
            cubic_bezier(res[0].t2, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1),
            res[0].coord
        );
        assert_eq!(
            cubic_bezier(res[0].t1, &c1_p0, &c1_cp0, &c1_cp1, &c1_p1),
            res[0].coord
        );
    }

    #[test]
    fn when_two_complex_curves_then_intersection() {
        let m = Affine::identity()
            .translate(ScreenCoord::new(-20.0, -20.0))
            .scale(ScreenLength2d::new(
                1.0 / (235.0 - 20.0),
                1.0 / (235.0 - 20.0),
            ));

        let c1_p0 = m * Coord::new(50.0, 35.0);
        let c1_cp0 = m * Coord::new(45.0, 235.0);
        let c1_cp1 = m * Coord::new(220.0, 235.0);
        let c1_p1 = m * Coord::new(220.0, 135.0);

        let c2_p0 = m * Coord::new(20.0, 150.0);
        let c2_cp0 = m * Coord::new(120.0, 20.0);
        let c2_cp1 = m * Coord::new(220.0, 95.0);
        let c2_p1 = m * Coord::new(140.0, 240.0);

        let res = intersection_simple(
            &c1_p0, &c1_cp0, &c1_cp1, &c1_p1, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1,
        );

        assert_eq!(res.len(), 2);
        assert!(res[0].t1 != res[1].t1);

        assert_eq!(
            cubic_bezier(res[0].t1, &c1_p0, &c1_cp0, &c1_cp1, &c1_p1),
            res[0].coord
        );
        assert_eq!(
            cubic_bezier(res[0].t2, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1),
            res[0].coord
        );

        assert_eq!(
            cubic_bezier(res[1].t1, &c1_p0, &c1_cp0, &c1_cp1, &c1_p1),
            res[1].coord
        );
        assert_eq!(
            cubic_bezier(res[1].t2, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1),
            res[1].coord
        );
    }

    #[test]
    fn when_two_complex_curves_then_intersection_3() {
        let m = Affine::identity()
            .translate(ScreenCoord::new(-20.0, -20.0))
            .scale(ScreenLength2d::new(
                1.0 / (235.0 - 20.0),
                1.0 / (235.0 - 20.0),
            ));

        let c1_p0 = m * Coord::new(50.0, 35.0);
        let c1_cp0 = m * Coord::new(45.0, 235.0);
        let c1_cp1 = m * Coord::new(220.0, 235.0);
        let c1_p1 = m * Coord::new(220.0, 135.0);

        let c2_p0 = m * Coord::new(81.0, 113.0);
        let c2_cp0 = m * Coord::new(20.0, 208.0);
        let c2_cp1 = m * Coord::new(220.0, 95.0);
        let c2_p1 = m * Coord::new(140.0, 240.0);

        let res = intersection_simple(
            &c1_p0, &c1_cp0, &c1_cp1, &c1_p1, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1,
        );

        assert_eq!(res.len(), 3);
        assert!(res[0].t1 != res[1].t1 && res[0].t1 != res[2].t1);

        assert_eq!(
            cubic_bezier(res[0].t1, &c1_p0, &c1_cp0, &c1_cp1, &c1_p1),
            res[0].coord
        );
        assert_eq!(
            cubic_bezier(res[0].t2, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1),
            res[0].coord
        );

        assert_eq!(
            cubic_bezier(res[1].t1, &c1_p0, &c1_cp0, &c1_cp1, &c1_p1),
            res[1].coord
        );
        assert_eq!(
            cubic_bezier(res[1].t2, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1),
            res[1].coord
        );

        assert_eq!(
            cubic_bezier(res[2].t1, &c1_p0, &c1_cp0, &c1_cp1, &c1_p1),
            res[2].coord
        );
        assert_eq!(
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
        let c1_p0 = Coord::new(0.0, 0.0);
        let c1_cp0 = Coord::new(0.0, 0.0);
        let c1_cp1 = Coord::new(-1.0, -1.0);
        let c1_p1 = Coord::new(-1.0, -1.0);

        let c2_p0 = Coord::new(-0.5679927, -0.56999993);
        let c2_cp0 = Coord::new(-0.56799376, -0.5693334);
        let c2_cp1 = Coord::new(-0.567997, -0.5686675);
        let c2_p1 = Coord::new(-0.56800234, -0.56800234);

        let res = intersection_simple(
            &c1_p0, &c1_cp0, &c1_cp1, &c1_p1, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1,
        );

        assert_eq!(res.len(), 1); //gets filtered out
    }

    #[test]
    fn inter() {
        let c1_p0 = Coord::new(1.0, -1.0);
        let c1_cp0 = Coord::new(1.0, -1.0);
        let c1_cp1 = Coord::new(1.0, 1.0);
        let c1_p1 = Coord::new(1.0, 1.0);

        let c2_p0 = Coord::new(0.0, 0.0);
        let c2_cp0 = Coord::new(0.0, 0.0);
        let c2_cp1 = Coord::new(1.0, 1.0);
        let c2_p1 = Coord::new(1.0, 1.0);

        let res = intersection_simple(
            &c1_p0, &c1_cp0, &c1_cp1, &c1_p1, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1,
        );

        assert_eq!(res.len(), 1);
    }

    #[test]
    fn bugg3() {
        //-0.7606782 -0.88851035 -0.7836163 -0.9487264 -0.8267758 -0.9891847 -0.8764745 -0.98936236
        //-0.8764745 -0.98936236 C -0.90502703 -0.98926026 -0.9314211 -0.975863 -0.9530624 -0.9530624

        let cu10 = vec![
            Coord::new(-0.7606782, -0.88851035),
            Coord::new(-0.7836163, -0.9487264),
            Coord::new(-0.8267758, -0.9891847),
            Coord::new(-0.8764745, -0.98936236),
        ];

        let cu11 = vec![
            Coord::new(-0.8764745, -0.98936236),
            Coord::new(-0.90502703, -0.98926026),
            Coord::new(-0.9314211, -0.975863),
            Coord::new(-0.9530624, -0.9530624),
        ];

        //-0.7431338 -0.78935134 C -0.74330974 -0.9000367 -0.80268425 -0.98909855 -0.8764745 -0.98936236
        //-0.8764745 -0.98936236 C -0.90502703 -0.98926026 -0.9314211 -0.975863 -0.9530624 -0.9530624

        let cu20 = vec![
            Coord::new(-0.7431338, -0.78935134),
            Coord::new(-0.74330974, -0.9000367),
            Coord::new(-0.80268425, -0.98909855),
            Coord::new(-0.8764745, -0.98936236),
        ];

        let cu21 = vec![
            Coord::new(-0.8764745, -0.98936236),
            Coord::new(-0.90502703, -0.98926026),
            Coord::new(-0.9314211, -0.975863),
            Coord::new(-0.9530624, -0.9530624),
        ];

        let res1020 = intersection(
            &cu10[0], &cu10[1], &cu10[2], &cu10[3], &cu20[0], &cu20[1], &cu20[2], &cu20[3],
        );
        assert!(matches!(res1020, IntersectionResult::ASmallerAndInsideB));

        let res1021 = intersection(
            &cu10[0], &cu10[1], &cu10[2], &cu10[3], &cu21[0], &cu21[1], &cu21[2], &cu21[3],
        );
        assert!(matches!(res1021, IntersectionResult::Pts(_)));

        let res1120 = intersection(
            &cu11[0], &cu11[1], &cu11[2], &cu11[3], &cu20[0], &cu20[1], &cu20[2], &cu20[3],
        );
        assert!(matches!(res1120, IntersectionResult::Pts(_)));

        let res1121 = intersection(
            &cu11[0], &cu11[1], &cu11[2], &cu11[3], &cu21[0], &cu21[1], &cu21[2], &cu21[3],
        );
        assert!(matches!(res1121, IntersectionResult::ASmallerAndInsideB));
    }

    #[test]
    fn given_curve_intersect_at_point_when_intersect_then_pts() {
        //-0.592 -0.8220109 -0.6349202 -0.82185745 -0.67296314 -0.7916621 -0.6973167 -0.74464357
        let cu11 = vec![
            Coord::new(-0.592, -0.8220109),
            Coord::new(-0.6349202, -0.82185745),
            Coord::new(-0.67296314, -0.7916621),
            Coord::new(-0.6973167, -0.74464357),
        ];

        //-0.72777134 -0.72777134 C -0.70948786 -0.741921 -0.6887212 -0.7499321 -0.66666675 -0.75001097

        let cu20 = vec![
            Coord::new(-0.72777134, -0.72777134),
            Coord::new(-0.70948786, -0.741921),
            Coord::new(-0.6887212, -0.7499321),
            Coord::new(-0.66666675, -0.75001097),
        ];

        let res1120 = intersection(
            &cu11[0], &cu11[1], &cu11[2], &cu11[3], &cu20[0], &cu20[1], &cu20[2], &cu20[3],
        );
        assert!(matches!(res1120, IntersectionResult::Pts(_)));
    }

    #[test]
    fn given_small_curve_when_extremites_then_3_pts() {
        //-0.592 -0.8220109 -0.6349202 -0.82185745 -0.67296314 -0.7916621 -0.6973167 -0.74464357
        let cu11 = vec![
            Coord::new(-0.697316706, -0.744643509),
            Coord::new(-0.697316646, -0.744643569),
            Coord::new(-0.697316646, -0.744643569),
            Coord::new(-0.697316586, -0.744643509),
        ];

        let vec = extremites(&cu11[0], &cu11[1], &cu11[2], &cu11[3]);
        assert_eq!(vec.len(), 3);
    }
}
