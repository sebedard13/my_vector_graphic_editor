use crate::pures::{Mat2x2, Vec2};
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

impl Coord {
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

/**
 * A transform matrix with offset
 */
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Transform {
    pub transform: Mat2x2,
    pub offset: Vec2,
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
