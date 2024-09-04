use float_cmp::{ApproxEq, F32Margin};
use serde::{Deserialize, Serialize};

use crate::{pures::Vec2, vec2_op};

/// A 2D vector representing a direction
#[derive(Clone, Debug, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Vector {
    x: f32,
    y: f32,
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
