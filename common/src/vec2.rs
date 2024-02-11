use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::forward_ref_binop;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Copy, Default)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normal(&self) -> Vec2 {
        let norm = self.norm();
        Vec2 {
            x: self.x / norm,
            y: self.y / norm,
        }
    }

    pub fn normalize(&mut self) {
        let norm = self.norm();
        self.x /= norm;
        self.y /= norm;
    }

    pub fn distance(&self, other: Vec2) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn approx_distance(&self, other: Vec2) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

forward_ref_binop!(impl Add, add for Vec2, Vec2);

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

forward_ref_binop!(impl Sub, sub for Vec2, Vec2);

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: f32) -> Vec2 {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

forward_ref_binop!(impl Mul, mul for Vec2, f32);

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, other: f32) -> Vec2 {
        Vec2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

forward_ref_binop!(impl Div, div for Vec2, f32);
