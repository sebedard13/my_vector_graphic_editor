use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use crate::curve::Curve;

use crate::instructions::{CoordInstruction, ShapeInstruction};
use crate::vgc_struct::Shape;

/* 
pub fn insert_curve(coord_ds: &mut CoordDS, curve_instruction: CoordInstruction) -> Curve {
    let c1 = coord_ds.insert(curve_instruction.c1);
    let c2 = coord_ds.insert(curve_instruction.c2);
    let p = coord_ds.insert(curve_instruction.p);
    Curve::new(c1, c2, p)
}

pub fn insert_shape(coord_ds: &mut CoordDS, shape_instruction: ShapeInstruction) -> Shape {
    let start = coord_ds.insert(shape_instruction.start);

    let curves: Vec<Curve> = shape_instruction
        .curves
        .iter()
        .map(|curve_instruction| {
            insert_curve(coord_ds, curve_instruction.clone()) //TODO: clone is not good
        })
        .collect();

    let mut shape = Shape {
        start,
        curves,
        color: shape_instruction.color,
    };
    shape.close();
    shape
}*/


#[derive(Clone, Debug, PartialEq)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
}
impl Coord {
    pub fn key(&self) -> u64 {
        let mut key:u64 = self.x.to_bits().into();
        key <<= 32;
        let into:u64 = self.y.to_bits().into();
        key |= into;
        key
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Coord {
        let norm = self.norm();
        Coord {
            x: self.x / norm,
            y: self.y / norm,
        }
    }

    pub fn distance(&self, curve_coord: &Coord) -> f32 {
        let dx = self.x - curve_coord.x;
        let dy = self.y - curve_coord.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Approximate distance, without sqrt
    pub fn approx_distance(&self, curve_coord: &Coord) -> f32 {
        let dx = self.x - curve_coord.x;
        let dy = self.y - curve_coord.y;
        dx * dx + dy * dy
    }

    pub fn scale(&self,scale_x:u32,scale_y:u32)->Coord{
        Coord{
            x:self.x*(scale_x as f32),
            y:self.y*(scale_y as f32)
        }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}


impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Coord> for &Coord {
    type Output = Coord;

    fn add(self, rhs: &Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: &Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Coord> for &Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}


impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<&Coord> for &Coord {
    type Output = Coord;

    fn sub(self, rhs: &Coord) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Coord> for f32 {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Mul<&Coord> for f32 {
    type Output = Coord;

    fn mul(self, rhs: &Coord) -> Self::Output {
        Coord {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Div<f32> for Coord {
    type Output = Coord;

    fn div(self, rhs: f32) -> Self::Output {
        Coord {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<f32> for &Coord {
    type Output = Coord;

    fn div(self, rhs: f32) -> Self::Output {
        Coord {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
