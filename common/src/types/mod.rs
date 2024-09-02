use crate::pures::{Affine, Vec2};

use serde::{Deserialize, Serialize};

use tsify_next::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

mod coord;
pub use coord::Coord;

/**
 * A screen coordinate in pixels
 */

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub struct ScreenCoord {
    pub c: Vec2,
}

impl ScreenCoord {
    #[cfg_attr(
        all(feature = "bindgen", not(feature = "ts")),
        wasm_bindgen(constructor)
    )]
    pub fn new(x: f32, y: f32) -> ScreenCoord {
        ScreenCoord { c: Vec2::new(x, y) }
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
    #[cfg_attr(
        all(feature = "bindgen", not(feature = "ts")),
        wasm_bindgen(constructor)
    )]
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Rect {
        Rect {
            top_left: Coord::new(x0, y0),
            bottom_right: Coord::new(x1, y1),
        }
    }

    pub fn width(&self) -> f32 {
        self.bottom_right.c.x - self.top_left.c.x
    }

    pub fn height(&self) -> f32 {
        self.bottom_right.c.y - self.top_left.c.y
    }

    pub fn center(&self) -> Coord {
        Coord::new(
            (self.top_left.c.x + self.bottom_right.c.x) / 2.0,
            (self.top_left.c.y + self.bottom_right.c.y) / 2.0,
        )
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        self.top_left.c.x <= other.bottom_right.c.x
            && self.bottom_right.c.x >= other.top_left.c.x
            && self.top_left.c.y <= other.bottom_right.c.y
            && self.bottom_right.c.y >= other.top_left.c.y
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
        self.top_left.c.x <= point.c.x
            && self.bottom_right.c.x >= point.c.x
            && self.top_left.c.y <= point.c.y
            && self.bottom_right.c.y >= point.c.y
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
    /// assert_eq!(rect.center().transform(&affine), Coord::new(0.0, 0.0));
    /// assert_eq!(rect.top_left.transform(&affine), Coord::new(-1.0, -1.0));
    /// assert_eq!(rect.bottom_right.transform(&affine), Coord::new(1.0, 1.0));
    /// ```
    pub fn affine_to_normal(&self) -> Affine {
        Affine::identity()
            .translate(self.center().c * -1.0)
            .scale(Vec2::new(
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
    #[cfg_attr(
        all(feature = "bindgen", not(feature = "ts")),
        wasm_bindgen(constructor)
    )]
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> ScreenRect {
        ScreenRect {
            top_left: ScreenCoord::new(x0, y0),
            bottom_right: ScreenCoord::new(x1, y1),
        }
    }

    pub fn width(&self) -> f32 {
        self.bottom_right.c.x - self.top_left.c.x
    }

    pub fn height(&self) -> f32 {
        self.bottom_right.c.y - self.top_left.c.y
    }

    pub fn center(&self) -> ScreenCoord {
        ScreenCoord::new(
            (self.top_left.c.x + self.bottom_right.c.x) / 2.0,
            (self.top_left.c.y + self.bottom_right.c.y) / 2.0,
        )
    }

    pub fn length(&self) -> ScreenLength2d {
        ScreenLength2d::new(self.width(), self.height())
    }
}

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub struct ScreenLength2d {
    pub c: Vec2,
}

impl ScreenLength2d {
    #[cfg_attr(
        all(feature = "bindgen", not(feature = "ts")),
        wasm_bindgen(constructor)
    )]
    pub fn new(x: f32, y: f32) -> ScreenLength2d {
        ScreenLength2d { c: Vec2::new(x, y) }
    }
}

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub struct Length2d {
    pub c: Vec2,
}

impl Length2d {
    #[cfg_attr(
        all(feature = "bindgen", not(feature = "ts")),
        wasm_bindgen(constructor)
    )]
    pub fn new(x: f32, y: f32) -> Length2d {
        Length2d { c: Vec2::new(x, y) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_affine_to_normal() {
        let rect = Rect::new(2.0, 7.0, 9.0, 9.0);
        let affine = rect.affine_to_normal();

        assert_eq!(rect.center().transform(&affine), Coord::new(0.0, 0.0));
        assert_eq!(rect.top_left.transform(&affine), Coord::new(-1.0, -1.0));
        assert_eq!(rect.bottom_right.transform(&affine), Coord::new(1.0, 1.0));
    }
}
