use crate::{forward_ref_binop, forward_ref_unop};
use crate::{pures::Vec2, vec2_op};
use float_cmp::{ApproxEq, F32Margin};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Neg, Sub};

use super::Coord;

/// A 2D vector representing a direction
#[derive(Clone, Debug, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector{
    pub fn new(x: f32, y: f32) -> Vector {
        Vector { x, y }
    }
}

impl Vec2 for Vector {
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

vec2_op!(Vector);

impl ApproxEq for Vector {
    type Margin = F32Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let epsilon = margin.into();
        self.x.approx_eq(other.x, epsilon) && self.y.approx_eq(other.y, epsilon)
    }
}

impl From<Vector> for Coord {
    fn from(vector: Vector) -> Coord {
        Coord::new(vector.x, vector.y)
    }
}
