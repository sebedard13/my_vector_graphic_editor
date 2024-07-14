use float_cmp::{ApproxEq, F32Margin};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};
#[cfg(feature = "ts")]
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{forward_ref_binop, forward_ref_unop};

#[cfg_attr(all(feature = "bindgen", not(feature = "ts")), wasm_bindgen)]
#[cfg_attr(feature = "ts", derive(Tsify))]
#[cfg_attr(feature = "ts", tsify(into_wasm_abi, from_wasm_abi))]
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

    pub fn min(a: &Vec2, b: &Vec2) -> Vec2 {
        Vec2 {
            x: a.x.min(b.x),
            y: a.y.min(b.y),
        }
    }

    pub fn max(a: &Vec2, b: &Vec2) -> Vec2 {
        Vec2 {
            x: a.x.max(b.x),
            y: a.y.max(b.y),
        }
    }

    pub fn prec_eq(&self, other: &Vec2) -> bool {
        let precision = f32::EPSILON * 100.0;
        (self.x - other.x).abs() < precision && (self.y - other.y).abs() < precision
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

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

forward_ref_binop!(impl Mul, mul for f32, Vec2);

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

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

forward_ref_binop!(impl Mul, mul for Vec2, Vec2);

impl Div<Vec2> for Vec2 {
    type Output = Vec2;

    fn div(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

forward_ref_binop!(impl Div, div for Vec2, Vec2);

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

forward_ref_unop!(impl Neg, neg for Vec2);

impl ApproxEq for &Vec2 {
    type Margin = F32Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x, margin) && self.y.approx_eq(other.y, margin)
    }
}

impl ApproxEq for Vec2 {
    type Margin = F32Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x, margin) && self.y.approx_eq(other.y, margin)
    }
}
