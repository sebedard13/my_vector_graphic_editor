use std::ops::{Add, Mul};
use crate::coord::{Coord, CoordIndex};

pub struct File {
    pub version: u32,
    pub background: RGBA,
    pub ratio: f64,
    pub regions: Vec<Region>,
}

pub struct Region {
    pub start: CoordIndex,
    pub curves: Vec<Curve>,
    pub color: RGBA,
}

pub struct Curve {
    pub c1: CoordIndex,
    pub c2: CoordIndex,
    pub p: CoordIndex,
}

pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}


impl Curve {
    pub fn new (c1: CoordIndex, c2: CoordIndex, p:CoordIndex)-> Curve{
        let mut c = Curve{c1,c2,p};

        return c;
    }

    /*pub fn evaluate(&self, t: f32, last_p: &Coord) -> Coord {
        if !(0.0 <= t && t <= 1.0) {
            panic!("Evaluate curve outside {}", t);
        }

        return cubic_bezier(t, last_p, &self.c1, &self.c2, &self.p);
    }*/
}

fn cubic_bezier(t: f32, p0: &Coord, p1: &Coord, p2: &Coord, p3: &Coord) -> Coord {
    return (1.0 - t) * quadratic_bezier(t, &p0, &p1, &p2) + t * quadratic_bezier(t, &p1, &p2, &p3);
}


fn quadratic_bezier(t: f32, p0: &Coord, p1: &Coord, p2: &Coord) -> Coord {
    let c = (1.0 - t) * (1.0 - t) * p0 + 2.0 * (1.0 - t) * t * p1 + t * t * p2;
    return c;
}


impl Mul<Coord> for f32 {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Self::Output {
        return Coord { x: self * rhs.x, y: self * rhs.y };
    }
}

impl Mul<&Coord> for f32 {
    type Output = Coord;

    fn mul(self, rhs: &Coord) -> Self::Output {
        return Coord { x: self * rhs.x, y: self * rhs.y };
    }
}