use std::f32::consts::PI;

use super::curve::tangent_vector;
use common::types::Coord;

#[allow(dead_code)]
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
    let range = (
        angle_of_curve(c1_p0, c1_cp0, c1_cp1, c1_p1, 1.0),
        angle_of_curve(c2_p1, c2_cp1, c2_cp0, c2_p0, 1.0),
    );
    let n_range = (
        angle_of_curve(n_c1_p0, n_c1_cp0, n_c1_cp1, n_c1_p1, 1.0),
        angle_of_curve(n_c2_p1, n_c2_cp1, n_c2_cp0, n_c2_p0, 1.0),
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
    !vector_together || has_a_same_angle
}

pub fn angle_of_curve(c1_p0: &Coord, c1_cp0: &Coord, c1_cp1: &Coord, c1_p1: &Coord, t: f32) -> f32 {
    let v0 = tangent_vector(t, c1_p1, c1_cp1, c1_cp0, c1_p0);
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
            angle_of_curve(&c1_p0, &c1_cp0, &c1_cp1, &c1_p1, 1.0),
            angle_of_curve(&c2_p1, &c2_cp1, &c2_cp0, &c2_p0, 1.0),
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
            angle_of_curve(&c1_p0, &c1_cp0, &c1_cp1, &c1_p1, 1.0),
            angle_of_curve(&c2_p1, &c2_cp1, &c2_cp0, &c2_p0, 1.0),
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
            angle_of_curve(&c1_p0, &c1_cp0, &c1_cp1, &c1_p1, 1.0),
            angle_of_curve(&c2_p1, &c2_cp1, &c2_cp0, &c2_p0, 1.0),
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
            angle_of_curve(&c1_p0, &c1_cp0, &c1_cp1, &c1_p1, 1.0),
            angle_of_curve(&c2_p1, &c2_cp1, &c2_cp0, &c2_p0, 1.0),
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

        let n_c1_p0 = Coord::new(0.0, 0.0);
        let n_c1_cp0 = Coord::new(0.0, 0.0);
        let n_c1_cp1 = Coord::new(1.0, 0.0);
        let n_c1_p1 = Coord::new(1.0, 0.0);
        let n_c2_p0 = Coord::new(1.0, 0.0);
        let n_c2_cp0 = Coord::new(1.0, 0.0);
        let n_c2_cp1 = Coord::new(2.0, 0.0);
        let n_c2_p1 = Coord::new(2.0, 0.0);

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
}
