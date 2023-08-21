use serde::{Deserialize, Serialize};
use crate::coord::{Coord, CoordIndex};

/// A curve is a cubic bezier curve, defined by 4 points:
/// - cp0 is the control point for the point before the current curve
/// - cp1 is the control point before the current point
/// - p1 is the current point
///
/// The curve is drawn from the previous curve point [i-1].p1, with [i].h1 and [i].h2 as control points and [i].p1 for the final points.
#[derive(Deserialize, Serialize, Debug)]
pub struct Curve {
    pub cp0: CoordIndex,
    pub cp1: CoordIndex,
    pub p1: CoordIndex,
}


impl Curve {
    pub fn new(c1: CoordIndex, c2: CoordIndex, p: CoordIndex) -> Curve {
        Curve {
            cp0: c1,
            cp1: c2,
            p1: p,
        }
    }
}

fn cubic_bezier(t: f32, c0: &Coord, h0: &Coord, h1: &Coord, p1: &Coord) -> Coord {
    (1.0 - t) * (1.0 - t) * (1.0 - t) * c0
        + 3.0 * (1.0 - t) * (1.0 - t) * t * h0
        + 3.0 * (1.0 - t) * t * t * h1
        + t * t * t * p1
}

fn cubic_bezier_derivative(t: f32, p0: &Coord, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Coord {
    3.0 * (1.0 - t) * (1.0 - t) * (cp0 - p0)
        + 6.0 * (1.0 - t) * t * (cp1 - cp0)
        + 3.0 * t * t * (p1 - cp1)
}

/// Return the normalized tangent vector at t of curve defined by p0, cp0, cp1, p1
/// Panic if no tangent vector found by having the same point for p0, cp0, cp1 and p1
fn tangent_vector(t: f32, p0: &Coord, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Coord {
    if p0 == p1 && p0 == cp0 && p0 == cp1 {
        return p0 - &Coord { x: 0.1, y: 0.1 };
    }

    let tangent_vector = cubic_bezier_derivative(t, p0, cp0, cp1, p1);
    if tangent_vector != (Coord { x: 0.0, y: 0.0 }) {
        //Normalize vector
        return tangent_vector.normalize();
    }

    //Exception with (t = 1 and cp1 == p1) or (t = 0 and cp0 == p0)
    let t = t.clamp(0.0001, 0.9999);

    let tangent_vector = cubic_bezier_derivative(t, p0, cp0, cp1, p1);
    if tangent_vector != (Coord { x: 0.0, y: 0.0 }) {
        //Normalize vector
        return tangent_vector.normalize();
    }

    panic!(
        "No tangent vector found for t: {}, c0: {:?}, h0: {:?}, h1: {:?}, c1: {:?}",
        t, p0, cp0, cp1, p1
    );
}

/// Return two control points to create a smooth curve at t of curve defined by p0, cp0, cp1, p1
/// if t = 0.0 or 1.0 use tangent_cornor_pts() to use the sum of vector of two curve
fn tangent_pts(t: f32, p0: &Coord, cp0: &Coord, cp1: &Coord, p1: &Coord) -> [Coord; 2] {
    if p0 == p1 && p0 == cp0 && p0 == cp1 {
        return [
            p0 - &Coord { x: 0.1, y: 0.1 },
            p0 + &Coord { x: 0.1, y: 0.1 },
        ];
    }

    let tangent_vector = tangent_vector(t, p0, cp0, cp1, p1);
    let coord = cubic_bezier(t, p0, cp0, cp1, p1);

    let t_at = {
        let t_x = (p0.x - p1.x).abs();
        let t_y = (p0.y - p1.y).abs();
        if t_x > t_y {
            t_x / 2.0
        } else {
            t_y / 2.0
        }
    };

    let rtn = [
        Coord {
            x: coord.x - t_at * tangent_vector.x,
            y: coord.y - t_at * tangent_vector.y,
        },
        Coord {
            x: coord.x + t_at * tangent_vector.x,
            y: coord.y + t_at * tangent_vector.y,
        },
    ];

    rtn
}

/// Return two control points to create a smooth curve at a ppoint of two curves (p1).
/// The first curve is defined by p0, cp0, cp1, p1
/// The second curve is defined by p1, cp2, cp3, p2
pub fn tangent_cornor_pts(
    p0: &Coord,
    cp0: &Coord,
    cp1: &Coord,
    p1: &Coord,
    cp2: &Coord,
    cp3: &Coord,
    p2: &Coord,
) -> [Coord; 2] {
    let tangent_vector_l = tangent_vector(1.0, p0, cp0, cp1, p1);
    let tangent_vector_r = tangent_vector(0.0, p1, cp2, cp3, p2);

    let tangent_vector = (tangent_vector_l + tangent_vector_r).normalize();

    let coord = p1;

    let t_at = {
        let mut array_distance = [
            (p0.x - p1.x).abs() / 2.0,
            (p0.y - p1.y).abs() / 2.0,
            (p1.x - p2.x).abs() / 2.0,
            (p1.y - p2.y).abs() / 2.0,
        ];

        array_distance.sort_by(|a, b| a.partial_cmp(b).expect("Should not be NaN"));

        array_distance[3]
    };

    [
        Coord {
            x: coord.x - t_at * tangent_vector.x,
            y: coord.y - t_at * tangent_vector.y,
        },
        Coord {
            x: coord.x + t_at * tangent_vector.x,
            y: coord.y + t_at * tangent_vector.y,
        },
    ]
}

#[cfg(test)]
mod test {
    use std::f32::consts::PI;

    use crate::coord::Coord;
    use crate::curve::{tangent_cornor_pts, tangent_vector};

    #[test]
    fn tangent_vector_same() {
        let p0 = Coord { x: 1.0, y: 1.0 };
        let cp0 = Coord { x: 1.0, y: 1.0 };
        let cp1 = Coord { x: 1.0, y: 1.0 };
        let p1 = Coord { x: 1.0, y: 1.0 };

        let tangent = tangent_vector(1.0, &p0, &cp0, &cp1, &p1);

        assert!(!tangent.x.is_nan());
        assert!(!tangent.y.is_nan());

        assert_ne!(tangent, Coord { x: 0.0, y: 0.0 });
    }

    #[test]
    fn tangent_vector_cornor() {
        let p0 = Coord { x: 1.0, y: 0.0 };
        let cp0 = Coord { x: 1.0, y: 0.0 };
        let cp1 = Coord { x: 0.0, y: 0.0 };
        let p1 = Coord { x: 0.0, y: 0.0 };
        let cp2 = Coord { x: 0.0, y: 0.0 };
        let cp3 = Coord { x: 0.0, y: 1.0 };
        let p2 = Coord { x: 0.0, y: 1.0 };

        let sin = (0.25*PI).sin(); // 45deg
        let result = [Coord { x: 0.5*sin, y: -0.5*sin }, Coord { x: -0.5*sin, y: 0.5*sin }];

        assert_eq!(
            tangent_cornor_pts(&p0, &cp0, &cp1, &p1, &cp2, &cp3, &p2),
            result
        );
    }
}
