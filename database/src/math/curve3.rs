use std::f32::consts::PI;

use super::{
    curve::{cubic_bezier, tangent_vector},
    line_different_side::points_are_different_side,
};
use common::types::Coord;

/// Check if 2 path of 2 curves are intersecting for real
/// In a simple case path one could be a V and path two a U, the intersection point is at the bottom of the V and the U.
/// With that if the angle of the V is smaller than the U than the path are not intersecting.
///
/// This function also handle the case where the intersection are not at the same point and are separated by common path.
pub fn curve_realy_intersect(
    c1_p0: &Coord,
    c1_cp0: &Coord,
    c1_cp1: &Coord,
    c1_p1: &Coord,
    c2_p0: &Coord,
    c2_cp0: &Coord,
    c2_cp1: &Coord,
    c2_p1: &Coord,
    n_c1_p0: &Coord,
    n_c1_cp0: &Coord,
    n_c1_cp1: &Coord,
    n_c1_p1: &Coord,
    n_c2_p0: &Coord,
    n_c2_cp0: &Coord,
    n_c2_cp1: &Coord,
    n_c2_p1: &Coord,
) -> bool {
    // Handle intersection at the same point
    if c1_p1 == n_c1_p1 && c2_p0 == n_c2_p0 && c1_p1 == c2_p0 {
        let range = (
            angle_of_curve(c1_p1, c1_cp1, c1_cp0, c1_p0, 0.0),
            angle_of_curve(c2_p0, c2_cp0, c2_cp1, c2_p1, 0.0),
        );
        let n_range = (
            angle_of_curve(n_c1_p1, n_c1_cp1, n_c1_cp0, n_c1_p0, 0.0),
            angle_of_curve(n_c2_p0, n_c2_cp0, n_c2_cp1, n_c2_p1, 0.0),
        );

        let mut vec: Vec<(usize, f32)> = vec![
            (0, range.0 % (2.0 * PI)),
            (0, range.1 % (2.0 * PI)),
            (1, n_range.0 % (2.0 * PI)),
            (1, n_range.1 % (2.0 * PI)),
        ];
        vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let has_a_same_angle = vec[0].1 == vec[1].1 || vec[1].1 == vec[2].1;
        //If vector are next to each other after the sort, then the curves are not intersecting
        let vector_together = vec[0].0 == vec[1].0 || vec[1].0 == vec[2].0;

        return !vector_together || has_a_same_angle;
    } else if c1_p1 == n_c1_p1 && c2_p0 == n_c2_p0 || c1_p1 == n_c2_p0 && c2_p0 == n_c1_p1 {
        let lv1 = tangent_vector(0.0, n_c1_p1, n_c1_cp1, n_c1_cp0, n_c1_p0);
        let lv2 = tangent_vector(0.0, n_c2_p0, n_c2_cp0, n_c2_cp1, n_c2_p1);
        let point0 = cubic_bezier(0.1, c1_p1, c1_cp1, c1_cp0, c1_p0);
        let point1 = cubic_bezier(0.1, c2_p0, c2_cp0, c2_cp1, c2_p1);

        println!("{:?} {:?} {:?} {:?} {:?} {:?}", point0, point1, n_c1_p1, lv1, n_c2_p0, lv2);
        let res = points_are_different_side(&point0, &point1, n_c1_p1, &lv1, n_c2_p0, &lv2);
        res
    } else {
        panic!("broo what is this");
    }
}

pub fn angle_of_curve(c1_p0: &Coord, c1_cp0: &Coord, c1_cp1: &Coord, c1_p1: &Coord, t: f32) -> f32 {
    let v0 = tangent_vector(t, c1_p0, c1_cp0, c1_cp1, c1_p1);

    let angle = v0.y.atan2(v0.x);
    if angle < 0.0 {
        angle + 2.0 * PI
    } else {
        angle
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;
    use common::types::Coord;
    use float_cmp::assert_approx_eq;

    #[test]
    fn given_0_90_when_angle_range_then() {
        let c1_p0 = Coord::new(1.0, 0.0);
        let c1_cp0 = Coord::new(1.0, 0.0);
        let c1_cp1 = Coord::new(0.0, 0.0);
        let c1_p1 = Coord::new(0.0, 0.0);
        let c2_p0 = Coord::new(0.0, 0.0);
        let c2_cp0 = Coord::new(0.0, 0.0);
        let c2_cp1 = Coord::new(0.0, 1.0);
        let c2_p1 = Coord::new(0.0, 1.0);

        let range = (
            angle_of_curve(&c1_p1, &c1_cp1, &c1_cp0, &c1_p0, 0.0),
            angle_of_curve(&c2_p0, &c2_cp0, &c2_cp1, &c2_p1, 0.0),
        );
        assert_eq!(range, (0.0, 0.5 * PI));
    }

    #[test]
    fn given_45_90_when_angle_range_then() {
        let val = (2.0_f64.sqrt() / 2.0) as f32;
        let c1_p0 = Coord::new(val, val);
        let c1_cp0 = Coord::new(val, val);
        let c1_cp1 = Coord::new(0.0, 0.0);
        let c1_p1 = Coord::new(0.0, 0.0);
        let c2_p0 = Coord::new(0.0, 0.0);
        let c2_cp0 = Coord::new(0.0, 0.0);
        let c2_cp1 = Coord::new(0.0, 1.0);
        let c2_p1 = Coord::new(0.0, 1.0);

        let range = (
            angle_of_curve(&c1_p1, &c1_cp1, &c1_cp0, &c1_p0, 0.0),
            angle_of_curve(&c2_p0, &c2_cp0, &c2_cp1, &c2_p1, 0.0),
        );
        assert_approx_eq!(f32, range.0, 0.25 * PI, ulps = 2);
        assert_approx_eq!(f32, range.1, 0.5 * PI, ulps = 2);
    }

    #[test]
    fn given_120_210_angle_when_angle_range_then() {
        let val120deg = ((-1.0 / 2.0) as f32, (3.0_f64.sqrt() / 2.0) as f32);
        let val210deg = ((-3.0_f64.sqrt() / 2.0) as f32, (-1.0 / 2.0) as f32);

        let c1_p0 = Coord::new(val120deg.0, val120deg.1);
        let c1_cp0 = Coord::new(val120deg.0, val120deg.1);
        let c1_cp1 = Coord::new(0.0, 0.0);
        let c1_p1 = Coord::new(0.0, 0.0);
        let c2_p0 = Coord::new(0.0, 0.0);
        let c2_cp0 = Coord::new(0.0, 0.0);
        let c2_cp1 = Coord::new(val210deg.0, val210deg.1);
        let c2_p1 = Coord::new(val210deg.0, val210deg.1);

        let range = (
            angle_of_curve(&c1_p1, &c1_cp1, &c1_cp0, &c1_p0, 0.0),
            angle_of_curve(&c2_p0, &c2_cp0, &c2_cp1, &c2_p1, 0.0),
        );
        assert_approx_eq!(f32, range.0, 0.66666666666 * PI, ulps = 1);
        assert_approx_eq!(f32, range.1, 1.16666666666 * PI, ulps = 1);
    }

    #[test]
    fn given_315_30_angle_when_angle_range_then() {
        let val30deg = ((3.0_f64.sqrt() / 2.0) as f32, (1.0 / 2.0) as f32);
        let val315deg = (
            (2.0_f64.sqrt() / 2.0) as f32,
            (-2.0_f64.sqrt() / 2.0) as f32,
        );

        let c1_p0 = Coord::new(val30deg.0, val30deg.1);
        let c1_cp0 = Coord::new(val30deg.0, val30deg.1);
        let c1_cp1 = Coord::new(0.0, 0.0);
        let c1_p1 = Coord::new(0.0, 0.0);
        let c2_p0 = Coord::new(0.0, 0.0);
        let c2_cp0 = Coord::new(0.0, 0.0);
        let c2_cp1 = Coord::new(val315deg.0, val315deg.1);
        let c2_p1 = Coord::new(val315deg.0, val315deg.1);

        let range = (
            angle_of_curve(&c1_p1, &c1_cp1, &c1_cp0, &c1_p0, 0.0),
            angle_of_curve(&c2_p0, &c2_cp0, &c2_cp1, &c2_p1, 0.0),
        );
        assert_approx_eq!(f32, range.0, 0.16666666 * PI, ulps = 4);
        assert_approx_eq!(f32, range.1, 5.49778714378, ulps = 4);
    }

    #[test]
    fn given_315_30_and_90_0_when_curve_realy_intersect_then_intersect() {
        let val30deg = ((3.0_f64.sqrt() / 2.0) as f32, (1.0 / 2.0) as f32);
        let val315deg = (
            (2.0_f64.sqrt() / 2.0) as f32,
            (-2.0_f64.sqrt() / 2.0) as f32,
        );

        let c1_p0 = Coord::new(val30deg.0, val30deg.1);
        let c1_cp0 = Coord::new(val30deg.0, val30deg.1);
        let c1_cp1 = Coord::new(0.0, 0.0);
        let c1_p1 = Coord::new(0.0, 0.0);
        let c2_p0 = Coord::new(0.0, 0.0);
        let c2_cp0 = Coord::new(0.0, 0.0);
        let c2_cp1 = Coord::new(val315deg.0, val315deg.1);
        let c2_p1 = Coord::new(val315deg.0, val315deg.1);

        let n_c1_p0 = Coord::new(1.0, 0.0);
        let n_c1_cp0 = Coord::new(1.0, 0.0);
        let n_c1_cp1 = Coord::new(0.0, 0.0);
        let n_c1_p1 = Coord::new(0.0, 0.0);
        let n_c2_p0 = Coord::new(0.0, 0.0);
        let n_c2_cp0 = Coord::new(0.0, 0.0);
        let n_c2_cp1 = Coord::new(0.0, 1.0);
        let n_c2_p1 = Coord::new(0.0, 1.0);

        let intersect = curve_realy_intersect(
            &c1_p0, &c1_cp0, &c1_cp1, &c1_p1, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1, &n_c1_p0,
            &n_c1_cp0, &n_c1_cp1, &n_c1_p1, &n_c2_p0, &n_c2_cp0, &n_c2_cp1, &n_c2_p1,
        );

        assert_eq!(intersect, true);
    }

    #[test]
    fn given_120_210_and_90_0_when_curve_realy_intersect_then_not_intersect() {
        let val120deg = ((-1.0 / 2.0) as f32, (3.0_f64.sqrt() / 2.0) as f32);
        let val210deg = ((-3.0_f64.sqrt() / 2.0) as f32, (-1.0 / 2.0) as f32);

        let c1_p0 = Coord::new(val120deg.0, val120deg.1);
        let c1_cp0 = Coord::new(val120deg.0, val120deg.1);
        let c1_cp1 = Coord::new(0.0, 0.0);
        let c1_p1 = Coord::new(0.0, 0.0);
        let c2_p0 = Coord::new(0.0, 0.0);
        let c2_cp0 = Coord::new(0.0, 0.0);
        let c2_cp1 = Coord::new(val210deg.0, val210deg.1);
        let c2_p1 = Coord::new(val210deg.0, val210deg.1);

        let n_c1_p0 = Coord::new(1.0, 0.0);
        let n_c1_cp0 = Coord::new(1.0, 0.0);
        let n_c1_cp1 = Coord::new(0.0, 0.0);
        let n_c1_p1 = Coord::new(0.0, 0.0);
        let n_c2_p0 = Coord::new(0.0, 0.0);
        let n_c2_cp0 = Coord::new(0.0, 0.0);
        let n_c2_cp1 = Coord::new(0.0, 1.0);
        let n_c2_p1 = Coord::new(0.0, 1.0);

        let intersect = curve_realy_intersect(
            &c1_p0, &c1_cp0, &c1_cp1, &c1_p1, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1, &n_c1_p0,
            &n_c1_cp0, &n_c1_cp1, &n_c1_p1, &n_c2_p0, &n_c2_cp0, &n_c2_cp1, &n_c2_p1,
        );

        assert_eq!(intersect, false);
    }

    #[test]
    fn given_180_90_and_90_0_when_curve_realy_intersect_then_intersect() {
        let c1_p0 = Coord::new(-1.0, 0.0);
        let c1_cp0 = Coord::new(-1.0, 0.0);
        let c1_cp1 = Coord::new(0.0, 0.0);
        let c1_p1 = Coord::new(0.0, 0.0);
        let c2_p0 = Coord::new(0.0, 0.0);
        let c2_cp0 = Coord::new(0.0, 0.0);
        let c2_cp1 = Coord::new(0.0, 1.0);
        let c2_p1 = Coord::new(0.0, 1.0);

        let n_c1_p0 = Coord::new(1.0, 0.0);
        let n_c1_cp0 = Coord::new(1.0, 0.0);
        let n_c1_cp1 = Coord::new(0.0, 0.0);
        let n_c1_p1 = Coord::new(0.0, 0.0);
        let n_c2_p0 = Coord::new(0.0, 0.0);
        let n_c2_cp0 = Coord::new(0.0, 0.0);
        let n_c2_cp1 = Coord::new(0.0, 1.0);
        let n_c2_p1 = Coord::new(0.0, 1.0);

        let intersect = curve_realy_intersect(
            &c1_p0, &c1_cp0, &c1_cp1, &c1_p1, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1, &n_c1_p0,
            &n_c1_cp0, &n_c1_cp1, &n_c1_p1, &n_c2_p0, &n_c2_cp0, &n_c2_cp1, &n_c2_p1,
        );

        assert_eq!(intersect, true);
    }

    #[test]
    fn curve_realy_intersect_bug01() {
        //({ x: -0.03336434, y: 0.4988649, { x: -0.29439563, y: 0.48123237, { x: -0.49939466, y: 0.26549923, { x: -0.5000276, y: 0.0 })
        //({ x: -0.03336434, y: 0.4988649, { x: -0.022339618, y: 0.49960953, { x: -0.011214197, y: 0.50000083, { x: 0.0, y: 0.5000276 }),
        //({ x: -0.03336434, y: 0.4988649, { x: -0.19401999, y: 0.39140478, { x: -0.2995361, y: 0.20848922, { x: -0.30003315, y: 0.0 }),
        //({ x: -0.03336434, y: 0.4988649, { x: 0.061849367, y: 0.56255424, { x: 0.1764331, y: 0.5997386, { x: 0.3, y: 0.60003316 } })
        let c1_p0 = Coord::new(-0.5000276, 0.0);
        let c1_cp0 = Coord::new(-0.49939466, 0.26549923);
        let c1_cp1 = Coord::new(-0.29439563, 0.48123237);
        let c1_p1 = Coord::new(-0.03336434, 0.4988649);
        let c2_p0 = Coord::new(-0.03336434, 0.4988649);
        let c2_cp0 = Coord::new(-0.022339618, 0.49960953);
        let c2_cp1 = Coord::new(-0.011214197, 0.50000083);
        let c2_p1 = Coord::new(0.0, 0.5000276);

        let n_c1_p0 = Coord::new(-0.30003315, 0.0);
        let n_c1_cp0 = Coord::new(-0.2995361, 0.20848922);
        let n_c1_cp1 = Coord::new(-0.19401999, 0.39140478);
        let n_c1_p1 = Coord::new(-0.03336434, 0.4988649);
        let n_c2_p0 = Coord::new(-0.03336434, 0.4988649);
        let n_c2_cp0 = Coord::new(0.061849367, 0.56255424);
        let n_c2_cp1 = Coord::new(0.1764331, 0.5997386);
        let n_c2_p1 = Coord::new(0.3, 0.60003316);

        let intersect = curve_realy_intersect(
            &c1_p0, &c1_cp0, &c1_cp1, &c1_p1, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1, &n_c1_p0,
            &n_c1_cp0, &n_c1_cp1, &n_c1_p1, &n_c2_p0, &n_c2_cp0, &n_c2_cp1, &n_c2_p1,
        );
        assert_eq!(intersect, true);
    }

    #[test]
    fn angle_of_curve_bug01() {
        //({ x: -0.03336434, y: 0.4988649, { x: -0.29439563, y: 0.48123237, { x: -0.49939466, y: 0.26549923, { x: -0.5000276, y: 0.0 })
        //({ x: -0.03336434, y: 0.4988649, { x: -0.022339618, y: 0.49960953, { x: -0.011214197, y: 0.50000083, { x: 0.0, y: 0.5000276 }),
        let c1_p0 = Coord::new(-0.5000276, 0.0);
        let c1_cp0 = Coord::new(-0.49939466, 0.26549923);
        let c1_cp1 = Coord::new(-0.29439563, 0.48123237);
        let c1_p1 = Coord::new(-0.03336434, 0.4988649);
        let c2_p0 = Coord::new(-0.03336434, 0.4988649);
        let c2_cp0 = Coord::new(-0.022339618, 0.49960953);
        let c2_cp1 = Coord::new(-0.011214197, 0.50000083);
        let c2_p1 = Coord::new(0.0, 0.5000276);

        let angles = (
            angle_of_curve(&c1_p1, &c1_cp1, &c1_cp0, &c1_p0, 0.0),
            angle_of_curve(&c2_p0, &c2_cp0, &c2_cp1, &c2_p1, 0.0),
        );
        assert!(
            0.83333337217 * PI < angles.0 && angles.0 < 1.1666666 * PI,
            "Value Was {}",
            angles.0 / PI
        );
        assert!(
            -0.1666666 * PI < angles.1 && angles.1 < 0.1666666 * PI,
            "Value Was {}",
            angles.1 / PI
        );
    }

    #[test]
    fn curve_realy_intersect_bug02() {
        //{ x: -0.7, y: 0.5,{ x: -0.7, y: 0.5,{ x: 0.7, y: 0.5,{ x: 0.7, y: 0.5 }
        //{ x: -0.7, y: -0.50000006,{ x: -0.7, y: -0.50000006,{ x: -0.7, y: -0.7,{ x: -0.7, y: -0.7 }
        //{ x: -0.7, y: 0.5,{ x: -0.7, y: 0.5,{ x: -0.7, y: 0.7,{ x: -0.7, y: 0.7 }
        //{ x: -0.7, y: -0.50000006,{ x: -0.7, y: -0.50000006,{ x: 0.7, y: -0.49999997,{ x: 0.7, y: -0.49999997 } }

        let c1_p0 = Coord::new(-0.7, 0.5);
        let c1_cp0 = Coord::new(-0.7, 0.5);
        let c1_cp1 = Coord::new(0.7, 0.5);
        let c1_p1 = Coord::new(0.7, 0.5);
        let c2_p0 = Coord::new(-0.7, -0.50000006);
        let c2_cp0 = Coord::new(-0.7, -0.50000006);
        let c2_cp1 = Coord::new(-0.7, -0.7);
        let c2_p1 = Coord::new(-0.7, -0.7);

        let n_c1_p0 = Coord::new(-0.7, 0.5);
        let n_c1_cp0 = Coord::new(-0.7, 0.5);
        let n_c1_cp1 = Coord::new(-0.7, 0.7);
        let n_c1_p1 = Coord::new(-0.7, 0.7);
        let n_c2_p0 = Coord::new(-0.7, -0.50000006);
        let n_c2_cp0 = Coord::new(-0.7, -0.50000006);
        let n_c2_cp1 = Coord::new(0.7, -0.49999997);
        let n_c2_p1 = Coord::new(0.7, -0.49999997);

        let intersect = curve_realy_intersect(
            &c1_p1, &c1_cp1, &c1_cp0, &c1_p0, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1, &n_c1_p1,
            &n_c1_cp1, &n_c1_cp0, &n_c1_p0, &n_c2_p0, &n_c2_cp0, &n_c2_cp1, &n_c2_p1,
        );

        assert_eq!(intersect, true);
    }

    #[test]
    fn curve_realy_intersect_bug03(){
        //({ x: 0.0, y: 630.0 },{ x: 0.0, y: 630.0 },{ x: 45.0, y: 630.0 },{ x: 45.0, y: 630.0 } }
        //{ x: 0.0, y: 672.5 },{ x: 0.0, y: 672.5 },{ x: 0.0, y: 675.0 },{ x: 0.0, y: 675.0 } }
        //{ x: 0.0, y: 630.0 },{ x: 0.0, y: 630.0 },{ x: -1.1444092e-5, y: 172.5 },{ x: -1.1444092e-5, y: 172.5 } }
        //{ x: 0.0, y: 672.5 },{ x: 0.0, y: 672.5 },{ x: 45.0, y: 672.5 },{ x: 45.0, y: 672.5 } })

        let c1_p0 = Coord::new(0.0, 630.0);
        let c1_cp0 = Coord::new(0.0, 630.0);
        let c1_cp1 = Coord::new(45.0, 630.0);
        let c1_p1 = Coord::new(45.0, 630.0);
        let c2_p0 = Coord::new(0.0, 672.5);
        let c2_cp0 = Coord::new(0.0, 672.5);
        let c2_cp1 = Coord::new(0.0, 675.0);
        let c2_p1 = Coord::new(0.0, 675.0);

        let n_c1_p0 = Coord::new(0.0, 630.0);
        let n_c1_cp0 = Coord::new(0.0, 630.0);
        let n_c1_cp1 = Coord::new(-1.1444092e-5, 172.5);
        let n_c1_p1 = Coord::new(-1.1444092e-5, 172.5);
        let n_c2_p0 = Coord::new(0.0, 672.5);
        let n_c2_cp0 = Coord::new(0.0, 672.5);
        let n_c2_cp1 = Coord::new(45.0, 672.5);
        let n_c2_p1 = Coord::new(45.0, 672.5);

        let intersect = curve_realy_intersect(
            &c1_p1, &c1_cp1, &c1_cp0, &c1_p0, &c2_p0, &c2_cp0, &c2_cp1, &c2_p1, &n_c1_p1,
            &n_c1_cp1, &n_c1_cp0, &n_c1_p0, &n_c2_p0, &n_c2_cp0, &n_c2_cp1, &n_c2_p1,
        );

        assert_eq!(intersect, true);
    }
}
