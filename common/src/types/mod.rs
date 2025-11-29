use crate::{
    pures::{Affine, Vec2},
    vec2_op,
};

use float_cmp::{ApproxEq, F32Margin};
use serde::{Deserialize, Serialize};

use crate::{forward_ref_binop, forward_ref_unop};
use std::ops::{Add, Div, Mul, Neg, Sub};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

mod coord;
pub use coord::Coord;
mod vector;
pub use vector::Vector;

/**
 * A screen coordinate in pixels
 */

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[derive(Clone, Debug, Default, PartialEq, Copy, Serialize, Deserialize)]
pub struct ScreenCoord {
    pub x: f32,
    pub y: f32,
}

impl ScreenCoord {
    pub fn new(x: f32, y: f32) -> ScreenCoord {
        ScreenCoord { x, y }
    }
}

vec2_op!(ScreenCoord);

impl Vec2 for ScreenCoord {
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

impl ApproxEq for ScreenCoord {
    type Margin = F32Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x, margin) && self.y.approx_eq(other.y, margin)
    }
}

impl From<Coord> for ScreenCoord {
    fn from(value: Coord) -> Self {
        ScreenCoord::new(value.x, value.y)
    }
}

/**
 * A rectangle in the 2D space of the canvas
 */

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub struct Rect {
    pub top_left: Coord,
    pub bottom_right: Coord,
}

impl Rect {
   
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Rect {
        Rect {
            top_left: Coord::new(x0, y0),
            bottom_right: Coord::new(x1, y1),
        }
    }

    pub fn width(&self) -> f32 {
        self.bottom_right.x - self.top_left.x
    }

    pub fn height(&self) -> f32 {
        self.bottom_right.y - self.top_left.y
    }

    pub fn center(&self) -> Coord {
        Coord::new(
            (self.top_left.x + self.bottom_right.x) / 2.0,
            (self.top_left.y + self.bottom_right.y) / 2.0,
        )
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        self.top_left.x <= other.bottom_right.x
            && self.bottom_right.x >= other.top_left.x
            && self.top_left.y <= other.bottom_right.y
            && self.bottom_right.y >= other.top_left.y
    }

    pub fn max(a: &Rect, b: &Rect) -> Rect {
        Rect {
            top_left: Coord::min(&a.top_left, &b.top_left),
            bottom_right: Coord::max(&a.bottom_right, &b.bottom_right),
        }
    }

    pub fn approx_diagonal(&self) -> f32 {
        let dx = self.width();
        let dy = self.height();
        dx * dx + dy * dy
    }

    pub fn contains(&self, point: &Coord) -> bool {
        self.top_left.x <= point.x
            && self.bottom_right.x >= point.x
            && self.top_left.y <= point.y
            && self.bottom_right.y >= point.y
    }

    /// Returns an affine transformation that maps the rectangle to the normal space
    /// (centered at 0,0 and with a width and height of 2)
    ///
    /// ```rust
    /// use common::types::{Rect, Coord};
    /// use common::pures::Affine;
    ///
    /// let rect = Rect::new(2.0, 7.0, 9.0, 9.0);
    /// let affine = rect.affine_to_normal();
    ///
    /// assert_eq!(affine * rect.center(), Coord::new(0.0, 0.0));
    /// assert_eq!(affine * rect.top_left, Coord::new(-1.0, -1.0));
    /// assert_eq!(affine * rect.bottom_right, Coord::new(1.0, 1.0));
    /// ```
    pub fn affine_to_normal(&self) -> Affine {
        Affine::identity()
            .translate(self.center() * -1.0)
            .scale(Coord::new(
                1.0 / (self.width() / 2.0),
                1.0 / (self.height() / 2.0),
            ))
    }
}

/**
 * A rectangle in the screen space
 */

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub struct ScreenRect {
    pub top_left: ScreenCoord,
    pub bottom_right: ScreenCoord,
}

impl ScreenRect {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> ScreenRect {
        ScreenRect {
            top_left: ScreenCoord::new(x0, y0),
            bottom_right: ScreenCoord::new(x1, y1),
        }
    }

    pub fn width(&self) -> f32 {
        self.bottom_right.x - self.top_left.x
    }

    pub fn height(&self) -> f32 {
        self.bottom_right.y - self.top_left.y
    }

    pub fn center(&self) -> ScreenCoord {
        ScreenCoord::new(
            (self.top_left.x + self.bottom_right.x) / 2.0,
            (self.top_left.y + self.bottom_right.y) / 2.0,
        )
    }

    pub fn length(&self) -> ScreenLength2d {
        ScreenLength2d::new(self.width(), self.height())
    }
}

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[derive(Clone, Debug, Default, PartialEq, Copy, Serialize, Deserialize)]
pub struct ScreenLength2d {
    pub x: f32,
    pub y: f32,
}

impl ScreenLength2d {
    pub fn new(x: f32, y: f32) -> ScreenLength2d {
        ScreenLength2d { x, y }
    }
}

impl Vec2 for ScreenLength2d {
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

vec2_op!(ScreenLength2d);

impl ApproxEq for ScreenLength2d {
    type Margin = F32Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x, margin) && self.y.approx_eq(other.y, margin)
    }
}

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[derive(Clone, Default, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub struct Length2d {
    pub x: f32,
    pub y: f32,
}

impl Length2d {
    
    pub fn new(x: f32, y: f32) -> Length2d {
        Length2d { x, y }
    }
}

impl Vec2 for Length2d {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn set_y(&mut self, value: f32) {
        self.y = value;
    }
}

vec2_op!(Length2d);

impl ApproxEq for Length2d {
    type Margin = F32Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x, margin) && self.y.approx_eq(other.y, margin)
    }
}

impl From<ScreenLength2d> for Length2d {
    fn from(value: ScreenLength2d) -> Self {
        Length2d::new(value.x, value.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_affine_to_normal() {
        let rect = Rect::new(2.0, 7.0, 9.0, 9.0);
        let affine = rect.affine_to_normal();

        assert_eq!(affine * rect.center(), Coord::new(0.0, 0.0));
        assert_eq!(affine * rect.top_left, Coord::new(-1.0, -1.0));
        assert_eq!(affine * rect.bottom_right, Coord::new(1.0, 1.0));
    }
}
