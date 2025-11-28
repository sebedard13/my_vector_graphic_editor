use crate::{pures::Vec2, vec2_op, PRECISION};
use float_cmp::ApproxEq;
use serde::{Deserialize, Serialize};

use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use super::{Length2d, ScreenCoord, Vector};

/**
 * A coordinate in the 2D space of the canvas
 * Mostly 0.0 to 1.0, for a square canvas
 */

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[derive(Clone, Debug, Copy, Default, Serialize, Deserialize)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
}

impl Coord {
   
    pub fn new(x: f32, y: f32) -> Coord {
        Coord { x, y }
    }

    pub fn scale(&self, x: f32, y: f32, scale_x: f32, scale_y: f32) -> Coord {
        let x = self.x * scale_x + x;
        let y = self.y * scale_y + y;

        Coord::new(x, y)
    }
}
use crate::{forward_ref_binop, forward_ref_unop};
use std::ops::{Add, Div, Mul, Neg, Sub};
vec2_op!(Coord);

impl Vec2 for Coord {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        f32::abs(self.x - other.x) <= PRECISION && f32::abs(self.y - other.y) <= PRECISION
    }
}

// impl PartialOrd for Coord {
//     fn partial_cmp(&self, other: &Coord) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Eq for Coord {}

// impl Ord for Coord {
//     fn cmp(&self, other: &Coord) -> std::cmp::Ordering {
//         let x_cmp = self.x.partial_cmp(&other.x);
//         if x_cmp != Some(std::cmp::Ordering::Equal) {
//             return x_cmp.expect("Coord should not have NaN values");
//         }

//         self.y.partial_cmp(&other.y).expect("Coord should not have NaN values")
//     }
// }

#[derive(Clone, Copy)]
pub struct MarginCoord(u32);

impl Default for MarginCoord {
    fn default() -> Self {
        MarginCoord(1)
    }
}

impl From<u32> for MarginCoord {
    fn from(m: u32) -> MarginCoord {
        MarginCoord(m)
    }
}

impl ApproxEq for Coord {
    type Margin = MarginCoord;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into().0 as f32;
        f32::abs(self.x - other.x) <= PRECISION * margin
            && f32::abs(self.y - other.y) <= PRECISION * margin
    }
}

impl From<Coord> for Vector {
    fn from(coord: Coord) -> Vector {
        Vector {
            x: coord.x,
            y: coord.y,
        }
    }
}

impl From<ScreenCoord> for Coord {
    fn from(coord: ScreenCoord) -> Coord {
        Coord::new(coord.x, coord.y)
    }
}

impl From<Length2d> for Coord {
    fn from(length: Length2d) -> Coord {
        Coord::new(length.x, length.y)
    }
}