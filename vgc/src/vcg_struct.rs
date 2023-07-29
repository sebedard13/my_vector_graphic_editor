use std::ops::{Mul};
use serde::{Deserialize, Serialize};
use crate::coord::{Coord, CoordIndex};

#[derive(Deserialize,Serialize, Debug)]
pub struct Shape {
    pub start: CoordIndex,
    pub curves: Vec<Curve>,
    pub color: RGBA,
}
#[derive(Deserialize,Serialize, Debug)]
pub struct Curve {
    pub c1: CoordIndex,
    pub c2: CoordIndex,
    pub p: CoordIndex,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}


impl Curve {
    pub fn new (c1: CoordIndex, c2: CoordIndex, p:CoordIndex)-> Curve{
        

        Curve{c1,c2,p}
    }

    /*pub fn evaluate(&self, t: f32, last_p: &Coord) -> Coord {
        if !(0.0 <= t && t <= 1.0) {
            panic!("Evaluate curve outside {}", t);
        }

        return cubic_bezier(t, last_p, &self.c1, &self.c2, &self.p);
    }*/
}

fn cubic_bezier(t: f32, p0: &Coord, p1: &Coord, p2: &Coord, p3: &Coord) -> Coord {
    (1.0 - t) * quadratic_bezier(t, p0, p1, p2) + t * quadratic_bezier(t, p1, p2, p3)
}


fn quadratic_bezier(t: f32, p0: &Coord, p1: &Coord, p2: &Coord) -> Coord {
    
    (1.0 - t) * (1.0 - t) * p0 + 2.0 * (1.0 - t) * t * p1 + t * t * p2
}


impl Mul<Coord> for f32 {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Self::Output {
        Coord { x: self * rhs.x, y: self * rhs.y }
    }
}

impl Mul<&Coord> for f32 {
    type Output = Coord;

    fn mul(self, rhs: &Coord) -> Self::Output {
        Coord { x: self * rhs.x, y: self * rhs.y }
    }
}