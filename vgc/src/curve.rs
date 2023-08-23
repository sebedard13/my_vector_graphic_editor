use crate::coord::{Coord, CoordIndex};
use serde::{Deserialize, Serialize};
/// A curve is a cubic bezier curve, defined by 4 points:
/// - cp0 is the control point for the point before the current curve
/// - cp1 is the control point before the current point
/// - p1 is the current point
///
/// The curve is drawn from the previous curve point [i-1].p1, with [i].cp1 and [i].cph2 as control points and [i].cp1 for the final points.
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

/// Find the closest point on a curve defined by p0, cp0, cp1, p1
/// It return the t value of the curve, the distance and the closest point
///
// Based on https://stackoverflow.com/questions/2742610/closest-point-on-a-cubic-bezier-curve#answer-57315396
pub fn t_closest(
    coord: &Coord,
    p0: &Coord,
    cp0: &Coord,
    cp1: &Coord,
    p1: &Coord,
) -> (f32, f32, Coord) {

    let a = -1.0*p0 + 3.0 * cp0 - 3.0 * cp1 + p1;
    let b = 3.0 * p0 - 6.0 * cp0 + 3.0 * cp1;
    let c = -3.0 * p0 + 3.0 * cp0;
    let d = p0 - coord;

    // function of approximate distance between coord and curve for t
    //d(t):=(a_x*t^(3)+b_x*t^(2)+c_x*t+d_x)^(2)+(a_y*t^(3)+b_y*t^(2)+c_y*t+d_y)^(2)

    // 
    //d(t) ▸ (a_x^(2)+a_y^(2))*t^(6)
    //+(2*a_x*b_x+2*a_y*b_y)*t^(5)
    //+(2*a_x*c_x+2*a_y*c_y+b_x^(2)+b_y^(2))*t^(4)
    //+(2*a_x*d_x+2*a_y*d_y+2*b_x*c_x+2*b_y*c_y)*t^(3)
    //+(2*b_x*d_x+2*b_y*d_y+c_x^(2)+c_y^(2))*t^(2)
    //+(2*c_x*d_x+2*c_y*d_y)*t
    //+d_x^(2)+d_y^(2)

    //(d(t),t) ▸ 
    let da = 6.0 * (a.x.powi(2) + a.y.powi(2)) as f64; //6*(a_x^(2)+a_y^(2))*t^(5)
    let db = (10.0*a.x*b.x + 10.0*a.y*b.y) as f64;// 10*(a_x*b_x+a_y*b_y)*t^(4)
    let dc = 4.0*(2.0*a.x*c.x + 2.0*a.y*c.y + b.x.powi(2) + b.y.powi(2)) as f64;// 4*(2*a_x*c_x+2*a_y*c_y+b_x^(2)+b_y^(2))*t^(3)
    let dd = 6.0*(a.x*d.x + a.y*d.y + b.x*c.x + b.y*c.y) as f64;// 6*(a_x*d_x+a_y*d_y+b_x*c_x+b_y*c_y)*t^(2)
    let de = 2.0*(2.0*(b.x*d.x + b.y*d.y) + c.x.powi(2) + c.y.powi(2)) as f64;// 2*(2*b_x*d_x+2*b_y*d_y+c_x^(2)+c_y^(2))*t
    let df = 2.0*(c.x*d.x + c.y*d.y) as f64;// 2*c_x*d_x+2*c_y*d_y


    //Division by da, because function accept only monic polynomials
    let vec = &[db/da, dc/da, dd/da, de/da, df/da];
    
    let mut real_roots =roots::find_roots_sturm(vec, &mut 1e-8f64);


    real_roots.push(Ok(0.0));
    real_roots.push(Ok(1.0));

    let mut min_distance = std::f32::MAX;
    let mut min_t = 0.0;
    let mut min = Coord { x: 0.0, y: 0.0 };
    real_roots.iter().filter_map(|x| 
        match x{
            Ok(r) => Some(*r as f32),
            _ => None
        }
    ).filter(|x|  {x >= &&0.0 && x <= &&1.0}).for_each(|t| {
        let curve_coord = cubic_bezier(t, p0, cp0, cp1, p1);
        let distance = coord.approx_distance(&curve_coord);
        if distance < min_distance {
            min_distance = distance;
            min_t = t;
            min = curve_coord;
        }
    });
 
    (min_t, min_distance.sqrt(), min)
}

/// Evaluate the point at t of curve defined by p0, cp0, cp1, p1
fn cubic_bezier(t: f32, p0: &Coord, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Coord {
    (1.0 - t) * (1.0 - t) * (1.0 - t) * p0
        + 3.0 * (1.0 - t) * (1.0 - t) * t * cp0
        + 3.0 * (1.0 - t) * t * t * cp1
        + t * t * t * p1
}

/// Evaluate the derivative or the slope at t of curve defined by p0, cp0, cp1, p1
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
    use std::time::Instant;

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

        let sin = (0.25 * PI).sin(); // 45deg
        let result = [
            Coord {
                x: 0.5 * sin,
                y: -0.5 * sin,
            },
            Coord {
                x: -0.5 * sin,
                y: 0.5 * sin,
            },
        ];

        assert_eq!(
            tangent_cornor_pts(&p0, &cp0, &cp1, &p1, &cp2, &cp3, &p2),
            result
        );
    }

    #[test]
    fn t_closest_cornor(){
        let coord = Coord { x: 0.0, y: 0.0 };
        let p0 = Coord { x: 0.0, y: 1.0 };
        let cp0 = Coord { x: 0.0, y: 0.0 };
        let cp1 = Coord { x: 0.0, y: 0.0 };
        let p1 = Coord { x: 1.0, y: 0.0 };


        let (t, distance, closest)  = super::t_closest(&coord, &p0, &cp0, &cp1, &p1);

        assert_eq!(t, 0.5);
        assert_eq!(distance, 0.176776695297);
        assert_eq!(closest, Coord { x: 0.125, y: 0.125 });
    }

    #[test]
    fn bench_approx_distance_to_curve_and_t_closest_cornor(){
        let coord = Coord { x: 0.0, y: 0.0 };
        let p0 = Coord { x: 0.0, y: 1.0 };
        let cp0 = Coord { x: 0.0, y: 0.0 };
        let cp1 = Coord { x: 0.0, y: 0.0 };
        let p1 = Coord { x: 1.0, y: 0.0 };

        let now_approx = Instant::now();
        for _ in 0..1000 {
            // approx_distance_to_curve by divinding the curve equaly to find an approximation of the closest point
           // let option = super::approx_distance_to_curve(&coord, &p0, &cp0, &cp1, &p1);
        }
        let elapsed_approx = now_approx.elapsed().as_micros();
        println!("approx_distance_to_curve: {:?} us", elapsed_approx);

        let now_t_closest = Instant::now();
        for _ in 0..1000 {
            let option = super::t_closest(&coord, &p0, &cp0, &cp1, &p1);
        }
        let elapsed_t_closest = now_t_closest.elapsed().as_micros();
        println!("t_closest: {:?} us", elapsed_t_closest);
        if elapsed_approx < elapsed_t_closest {
           let percent = (elapsed_t_closest as f32 / elapsed_approx as f32) * 100.0;
            println!("approx_distance_to_curve is {}% faster", percent);
        }else{
            let percent = (elapsed_approx as f32 / elapsed_t_closest as f32) * 100.0;
            println!("t_closest is {}% faster", percent);
        }
       
        // The approx_distance_to_curve is faster if the number of iteration is not to big
        // at 20, 358% faster
        // at 50, 151% faster
        // at 100, 128% slower
        // the approx is not that good and create a grabing effect in the corner of two curve
    }
}
