use serde::{Deserialize, Serialize};

use crate::coord::{Coord, CoordIndex};

#[derive(Deserialize,Serialize, Debug)]
pub struct Shape {
    pub start: CoordIndex,
    pub curves: Vec<Curve>,
    pub color: Rgba,
}


// A curve is a cubic bezier curve, defined by 4 points:
// - cp0 is the control point for the point before the current curve
// - cp1 is the control point before the current point
// - p1 is the current point
//
// The curve is drawn from the previous curve point [i-1].p1, with [i].h1 and [i].h2 as control points and [i].p1 for the final points.
#[derive(Deserialize,Serialize, Debug)]
pub struct Curve {
    pub cp0: CoordIndex,
    pub cp1: CoordIndex,
    pub p1: CoordIndex,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}


impl Curve {
    pub fn new (c1: CoordIndex, c2: CoordIndex, p:CoordIndex)-> Curve{
        Curve { cp0: c1, cp1: c2, p1: p }
    }

    /*pub fn evaluate(&self, t: f32, last_p: &Coord) -> Coord {
        if !(0.0 <= t && t <= 1.0) {
            panic!("Evaluate curve outside {}", t);
        }

        return cubic_bezier(t, last_p, &self.c1, &self.c2, &self.p);
    }*/
}

fn cubic_bezier(t: f32, c0: &Coord, h0: &Coord, h1: &Coord, p1: &Coord) -> Coord {
    (1.0 - t) * (1.0 - t) * (1.0 - t) * c0 + 3.0 * (1.0 - t) * (1.0 - t) * t * h0 + 3.0 * (1.0 - t) * t * t * h1 + t * t * t * p1
}


fn cubic_bezier_derivative(t: f32, c0: &Coord, h0: &Coord, h1: &Coord, c1: &Coord) -> Coord {
    3.0 * (1.0 - t) * (1.0 - t) * (h0 - c0) + 6.0 * (1.0 - t) * t * (h1 - h0) + 3.0 * t * t * (c1 - h1)
}

fn tangent_pts(t: f32, c0: &Coord, h0: &Coord, h1: &Coord, c1: &Coord) -> [Coord; 2] {
    let vector = cubic_bezier_derivative(t, c0, h0, h1, c1);
    let coord = cubic_bezier(t, c0, h0, h1, c1);

    let t_at = {
        let t_x = (c0.x - c1.x).abs();
        let t_y = (c0.y - c1.y).abs();
        if t_x > t_y {
            t_x / 2.0
        } else {
            t_y / 2.0
        }
    };

    let vector = &vector / (vector.x.powi(2) + vector.y.powi(2)).sqrt();

    [
    Coord { x: coord.x + t_at * vector.x, y: coord.y + t_at * vector.y },
    Coord { x: coord.x - t_at * vector.x, y: coord.y - t_at * vector.y }
    ]
}