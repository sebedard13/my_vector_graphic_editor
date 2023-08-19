use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

use crate::instructions::{CurveInstruction, ShapeInstruction};
use crate::vcg_struct::{Curve, Shape};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CoordIndex {
    pub i: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CoordDS {
    pub array: Vec<Option<Coord>>,
    pub is_normalize: bool,
}

impl Default for CoordDS {
    fn default() -> Self {
        CoordDS {
            is_normalize: true,
            array: Vec::default(),
        }
    }
}

impl CoordDS {
    pub fn new() -> Self {
        CoordDS::default()
    }

    pub fn insert(&mut self, c: Coord) -> CoordIndex {
        self.array.push(Some(c));
        CoordIndex {
            i: self.array.len() - 1,
        }
    }

    pub fn get(&self, coord_index: &CoordIndex) -> &Coord {
        self.array[coord_index.i]
            .as_ref()
            .expect("Coord should be valid from CoordIndex")
    }

    pub fn modify(&mut self, coord_index: usize, c: Coord) {
        //TODO couple with CoordIndex?
        self.array[coord_index] = Some(c);
    }

    pub fn scale(&self, w: f32, h: f32) -> Self {
        let mut arr = self.array.clone();

        arr.iter_mut().for_each(|op_c| {
            match op_c {
                Some(c) => {
                    c.x *= w;
                    c.y *= h;
                }
                None => {}
            };
        });

        CoordDS {
            array: arr,
            is_normalize: false,
        }
    }
}

pub fn insert_curve(coord_ds: &mut CoordDS, curve_instruction: CurveInstruction) -> Curve {
    let c1 = coord_ds.insert(curve_instruction.c1);
    let c2 = coord_ds.insert(curve_instruction.c2);
    let p = coord_ds.insert(curve_instruction.p);
    Curve::new(c1, c2, p)
}

pub fn insert_shape(coord_ds: &mut CoordDS, shape_instruction: ShapeInstruction) -> Shape {
    let start = coord_ds.insert(shape_instruction.start);

    let mut curves: Vec<Curve> = shape_instruction
        .curves
        .iter()
        .map(|curve_instruction| {
            insert_curve(coord_ds, curve_instruction.clone()) //TODO: clone is not good
        })
        .collect();

    //Create last curve at start point for closing shape
    curves.push(Curve {
        cp0: start.clone(),
        cp1: start.clone(),
        p1: start.clone(),
    }); //TODO: clone is not good

    Shape {
        start,
        curves,
        color: shape_instruction.color,
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::coord::{Coord, CoordDS};

    #[test]
    fn scale_coord_ds() {
        let mut cds = CoordDS::new();
        cds.insert(Coord { x: 0.5, y: 0.2 });

        let sc_cds = cds.scale(10.0, 5.0);

        assert!(approx_eq!(
            f32,
            sc_cds.array[0].as_ref().unwrap().x,
            5.0,
            ulps = 2
        ));
        assert!(approx_eq!(
            f32,
            sc_cds.array[0].as_ref().unwrap().y,
            1.0,
            ulps = 2
        ));
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
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
