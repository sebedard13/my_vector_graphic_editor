use crate::{
    pures::{Affine, Vec2},
    PRECISION,
};
use float_cmp::{ApproxEq, F32Margin};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Neg, Sub};
use tsify_next::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{forward_ref_binop, forward_ref_unop};

/**
 * A coordinate in the 2D space of the canvas
 * Mostly 0.0 to 1.0, for a square canvas
 */

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[derive(Clone, Debug, Copy, Default, Serialize, Deserialize)]
pub struct Coord {
    pub c: Vec2,
}

impl Coord {
    #[cfg_attr(
        all(feature = "bindgen", not(feature = "ts")),
        wasm_bindgen(constructor)
    )]
    pub fn new(x: f32, y: f32) -> Coord {
        Coord { c: Vec2::new(x, y) }
    }

    pub fn x(&self) -> f32 {
        self.c.x
    }

    pub fn y(&self) -> f32 {
        self.c.y
    }

    pub fn set_x(&mut self, x: f32) {
        self.c.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.c.y = y;
    }

    pub fn scale(&self, x: f32, y: f32, scale_x: f32, scale_y: f32) -> Coord {
        let x = self.c.x * scale_x + x;
        let y = self.c.y * scale_y + y;

        Coord::new(x, y)
    }

    pub fn transform(&self, m: &Affine) -> Coord {
        let c = m * self.c;

        Coord { c }
    }

    pub fn min(a: &Coord, b: &Coord) -> Coord {
        Coord {
            c: Vec2::min(&a.c, &b.c),
        }
    }

    pub fn max(a: &Coord, b: &Coord) -> Coord {
        Coord {
            c: Vec2::max(&a.c, &b.c),
        }
    }

    pub fn norm(&self) -> f32 {
        self.c.norm()
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        f32::abs(self.c.x - other.c.x) <= PRECISION && f32::abs(self.c.y - other.c.y) <= PRECISION
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            c: self.c + other.c,
        }
    }
}

forward_ref_binop!(impl Add, add for Coord, Coord);

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Coord {
        Coord {
            c: self.c - other.c,
        }
    }
}

forward_ref_binop!(impl Sub, sub for Coord, Coord);

impl Mul<f32> for Coord {
    type Output = Coord;

    fn mul(self, other: f32) -> Coord {
        Coord { c: self.c * other }
    }
}

forward_ref_binop!(impl Mul, mul for Coord, f32);

impl Mul<Coord> for f32 {
    type Output = Coord;

    fn mul(self, other: Coord) -> Coord {
        Coord { c: self * other.c }
    }
}

forward_ref_binop!(impl Mul, mul for f32, Coord);

impl Div<f32> for Coord {
    type Output = Coord;

    fn div(self, other: f32) -> Coord {
        Coord { c: self.c / other }
    }
}

forward_ref_binop!(impl Div, div for Coord, f32);

impl Neg for Coord {
    type Output = Coord;

    fn neg(self) -> Coord {
        Coord { c: -self.c }
    }
}

forward_ref_unop!(impl Neg, neg for Coord);

impl ApproxEq for Coord {
    type Margin = F32Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.c.approx_eq(other.c, margin)
    }
}

impl ApproxEq for &Coord {
    type Margin = F32Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.c.approx_eq(other.c, margin)
    }
}
