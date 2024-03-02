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
    let res = recursive_intersection(c1_p0, c1_cp0, c1_cp1, c1_p1, c2_p0, c2_cp0, c2_cp1, c2_p1);

    // //Remove same coord if intersection is at t=0.5
    // let mut t_equal: u8 = 0b000;
    // let mut result = Vec::new();
    // for i in 0..res.len() {
    //     if res[i].t1 != 0.5 && res[i].t2 != 0.5 {
    //         result.push(res[i])
    //     }

    //     if res[i].t1 == 0.5 && res[i].t2 == 0.5 {
    //         if t_equal & 0b001 == 0b001 {
    //         } else {
    //             t_equal |= 0b001;
    //             result.push(res[i]);
    //         }
    //     } else if res[i].t1 == 0.5 {
    //         if t_equal & 0b010 == 0b010 {
    //         } else {
    //             t_equal |= 0b010;
    //         }
    //     } else if res[i].t2 == 0.5 {
    //         if t_equal & 0b100 == 0b100 {
    //         } else {
    //             t_equal |= 0b100;
    //             result.push(res[i]);
    //         }
    //     }
    // }

    res
}

fn recursive_intersection(
    c1_p0: &Coord,
    c1_cp0: &Coord,
    c1_cp1: &Coord,
    c1_p1: &Coord,
    c2_p0: &Coord,
    c2_cp0: &Coord,
    c2_cp1: &Coord,
    c2_p1: &Coord,
) -> Vec<IntersectionPoint> {
    let c1_rect = bounding_box(c1_p0, c1_cp0, c1_cp1, c1_p1);
    let c2_rect = bounding_box(c2_p0, c2_cp0, c2_cp1, c2_p1);

    if !own_intersect(&c1_rect, &c2_rect) {
        // println!("UP empty intersection");
        return Vec::new();
    }

    let max = &Rect::max(&c1_rect, &c2_rect);
    if max.approx_diagonal() < f32::EPSILON * f32::EPSILON {
        //* 10000000000.0
        let rtn = IntersectionPoint {
            coord: max.center(),
            t1: 0.5,
            t2: 0.5,
        };
        // println!("UP {:?}", rtn);
        return vec![rtn];
    }

    let (c1_1_cp0, c1_1_cp1, c1_1_p1, c1_2_cp0, c1_2_cp1) =
        add_smooth_result(c1_p0, c1_cp0, c1_cp1, c1_p1, 0.5);
    let c1_1_p0 = c1_p0;
    let c1_2_p0 = c1_1_p1;
    let c1_2_p1 = c1_p1;

    let (c2_1_cp0, c2_1_cp1, c2_1_p1, c2_2_cp0, c2_2_cp1) =
        add_smooth_result(c2_p0, c2_cp0, c2_cp1, c2_p1, 0.5);
    let c2_1_p0 = c2_p0;
    let c2_2_p0 = c2_1_p1;
    let c2_2_p1 = c2_p1;

    // println!(
    //     "Down 1.1 2.1 with {:?} to {:?} and {:?} to {:?}",
    //     c1_1_p0, c1_1_p1, c2_1_p0, c2_1_p1
    // );
    let res_c1_1_c2_1 = recursive_intersection(
        &c1_1_p0, &c1_1_cp0, &c1_1_cp1, &c1_1_p1, &c2_1_p0, &c2_1_cp0, &c2_1_cp1, &c2_1_p1,
    );
    // println!(
    //     "Down 1.1 2.2 with {:?} to {:?} and {:?} to {:?}",
    //     c1_1_p0, c1_1_p1, c2_2_p0, c2_2_p1
    // );
    let res_c1_1_c2_2 = recursive_intersection(
        &c1_1_p0, &c1_1_cp0, &c1_1_cp1, &c1_1_p1, &c2_2_p0, &c2_2_cp0, &c2_2_cp1, &c2_2_p1,
    );
    // println!(
    //     "Down 1.2 2.1 with {:?} to {:?} and {:?} to {:?}",
    //     c1_2_p0, c1_2_p1, c2_1_p0, c2_1_p1
    // );
    let res_c1_2_c2_1 = recursive_intersection(
        &c1_2_p0, &c1_2_cp0, &c1_2_cp1, &c1_2_p1, &c2_1_p0, &c2_1_cp0, &c2_1_cp1, &c2_1_p1,
    );
    // println!(
    //     "Down 1.2 2.2 with {:?} to {:?} and {:?} to {:?}",
    //     c1_2_p0, c1_2_p1, c2_2_p0, c2_2_p1
    // );
    let res_c1_2_c2_2 = recursive_intersection(
        &c1_2_p0, &c1_2_cp0, &c1_2_cp1, &c1_2_p1, &c2_2_p0, &c2_2_cp0, &c2_2_cp1, &c2_2_p1,
    );

    let mut rtn = Vec::new();

    for mut res in res_c1_1_c2_1 {
        res.t1 /= 2.0;
        res.t2 /= 2.0;

        rtn.push(res);
    }

    for mut res in res_c1_1_c2_2 {
        res.t1 /= 2.0;
        res.t2 /= 2.0;
        res.t2 += 0.5;

        rtn.push(res);
    }

    for mut res in res_c1_2_c2_1 {
        res.t1 /= 2.0;
        res.t1 += 0.5;
        res.t2 /= 2.0;

        rtn.push(res);
    }

    for mut res in res_c1_2_c2_2 {
        res.t1 /= 2.0;
        res.t1 += 0.5;
        res.t2 /= 2.0;
        res.t2 += 0.5;

        rtn.push(res);
    }

    // println!("UP {:?}", rtn);
    return rtn;
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
    }

    #[test]
    fn when_two_complex_curves_then_intersection() {
        let m = Affine::identity()
            .translate(Vec2::new(-50.0, -35.0))
            .scale(Vec2::new(1.0 / (231.0 - 50.0), 1.0 / (231.0 - 50.0)));

        let c1_p0 = Coord::new(50.0, 35.0).transform(&m);
        let c1_cp0 = Coord::new(41.0, 231.0).transform(&m);
        let c1_cp1 = Coord::new(186.0, 192.0).transform(&m);
        let c1_p1 = Coord::new(220.0, 135.0).transform(&m);

        let c2_p0 = Coord::new(88.0, 56.0).transform(&m);
        let c2_cp0 = Coord::new(41.0, 231.0).transform(&m);
        let c2_cp1 = Coord::new(125.0, 91.0).transform(&m);
        let c2_p1 = Coord::new(138.0, 216.0).transform(&m);

        let res = intersection(
            &c1_p0, &c1_cp0, &c1_cp1, &c1_p1, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1,
        );

        println!("{:?}", res);

     
        assert_eq!(res.len(), 3);
        assert!(res[0].t1 != res[1].t1 && res[0].t1 != res[2].t1);
    }
}
