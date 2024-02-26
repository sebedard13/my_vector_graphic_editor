use crate::pures::{Affine, Vec2};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

/**
 * A coordinate in the 2D space of the canvas
 * Mostly 0.0 to 1.0, for a square canvas
 */
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Copy, Default)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Coord {
    pub c: Vec2,
}

#[wasm_bindgen]
impl Coord {
    #[wasm_bindgen(constructor)]
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
}

/**
 * A screen coordinate in pixels
 */
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct ScreenCoord {
    pub c: Vec2,
}

#[wasm_bindgen]
impl ScreenCoord {
    #[wasm_bindgen(constructor)]
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
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Rect {
    pub top_left: Coord,
    pub bottom_right: Coord,
}
#[wasm_bindgen]
impl Rect {
    #[wasm_bindgen(constructor)]
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
}

/**
 * A rectangle in the screen space
 */
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct ScreenRect {
    pub top_left: ScreenCoord,
    pub bottom_right: ScreenCoord,
}
#[wasm_bindgen]
impl ScreenRect {
    #[wasm_bindgen(constructor)]
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

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct ScreenLength2d {
    pub c: Vec2,
}

#[wasm_bindgen]
impl ScreenLength2d {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> ScreenLength2d {
        ScreenLength2d { c: Vec2::new(x, y) }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Length2d {
    pub c: Vec2,
}
#[wasm_bindgen]
impl Length2d {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Length2d {
        Length2d { c: Vec2::new(x, y) }
    }
}
