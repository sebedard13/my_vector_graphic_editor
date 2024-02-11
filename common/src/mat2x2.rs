use std::ops::Mul;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::forward_ref_binop;
use crate::vec2::Vec2;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Mat2x2 {
    pub m00: f32,
    pub m01: f32,
    pub m10: f32,
    pub m11: f32,
}

impl Mat2x2 {
    pub fn new(m00: f32, m01: f32, m10: f32, m11: f32) -> Mat2x2 {
        Mat2x2 {
            m00,
            m01,
            m10,
            m11,
        }
    }

    pub fn identity() -> Mat2x2 {
        Mat2x2 {
            m00: 1.0,
            m01: 0.0,
            m10: 0.0,
            m11: 1.0,
        }
    }

    pub fn inverse(&self) -> Mat2x2 {
        Mat2x2 {
            m00: self.m11,
            m01: -self.m01,
            m10: -self.m10,
            m11: self.m00,
        }
    }

    pub fn from_rotation(angle: f32) -> Mat2x2 {
        let (s, c) = angle.sin_cos();
        Mat2x2 {
            m00: c,
            m01: -s,
            m10: s,
            m11: c,
        }
    }

    pub fn scale(&self, x: f32, y: f32) -> Mat2x2 {
        Mat2x2 {
            m00: self.m00 * x,
            m01: self.m01 * y,
            m10: self.m10 * x,
            m11: self.m11 * y,
        }
    }
}

impl Mul<Mat2x2> for Mat2x2 {
    type Output = Mat2x2;

    fn mul(self, rhs: Mat2x2)-> Mat2x2{
        Mat2x2 {
            m00: self.m00 * rhs.m00 + self.m01 * rhs.m10,
            m01: self.m00 * rhs.m01 + self.m01 * rhs.m11,
            m10: self.m10 * rhs.m00 + self.m11 * rhs.m10,
            m11: self.m10 * rhs.m01 + self.m11 * rhs.m11
        }
    }
}

forward_ref_binop!(impl Mul, mul for Mat2x2, Mat2x2);

impl Mul<Vec2> for Mat2x2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.m00 * rhs.x + self.m01 * rhs.y,
            y: self.m10 * rhs.x + self.m11 * rhs.y,
        }
    }
}

forward_ref_binop!(impl Mul, mul for Mat2x2, Vec2);