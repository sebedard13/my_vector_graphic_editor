use std::{fmt::Display, ops::Mul};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::forward_ref_binop;

use super::Vec2;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
// Matrix is stored in column major order
// Transformation matrix for 2D space
pub struct Mat2x3 {
    /// m00 m10 m20
    /// m01 m11 m21
    ///  0   0   1
    pub m00: f32,
    pub m01: f32,
    pub m10: f32,
    pub m11: f32,
    pub m20: f32,
    pub m21: f32,
}

impl Display for Mat2x3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{},{})\n({},{},{})",
            self.m00, self.m10, self.m20, self.m01, self.m11, self.m21
        )
    }
}

impl Mat2x3 {
    pub fn new(m00: f32, m01: f32, m10: f32, m11: f32, m20: f32, m21: f32) -> Mat2x3 {
        Mat2x3 {
            m00,
            m01,
            m10,
            m11,
            m20,
            m21,
        }
    }

    pub fn identity() -> Mat2x3 {
        Mat2x3 {
            m00: 1.0,
            m01: 0.0,
            m10: 0.0,
            m11: 1.0,
            m20: 0.0,
            m21: 0.0,
        }
    }

    pub fn inverse(&self) -> Mat2x3 {
        let inv_det = 1.0 / (self.m00 * self.m11 - self.m01 * self.m10);

        let new_m00 = self.m11 * inv_det;
        let new_m01 = -self.m01 * inv_det;
        let new_m10 = -self.m10 * inv_det;
        let new_m11 = self.m00 * inv_det;
        let new_m20 = -(self.m11 * self.m20 - self.m10 * self.m21) * inv_det;
        let new_m21 = (self.m01 * self.m20 - self.m00 * self.m21) * inv_det;

        Mat2x3 {
            m00: new_m00,
            m01: new_m01,
            m10: new_m10,
            m11: new_m11,
            m20: new_m20,
            m21: new_m21,
        }
    }

    pub fn get_scale(&self) -> Vec2 {
        Vec2::new(self.m00, self.m11)
    }

    pub fn get_translation(&self) -> Vec2 {
        Vec2::new(self.m20, self.m21)
    }

    pub fn from_rotation(angle: f32) -> Mat2x3 {
        let (s, c) = angle.sin_cos();
        Mat2x3 {
            m00: c,
            m01: -s,
            m10: s,
            m11: c,
            m20: 0.0,
            m21: 0.0,
        }
    }

    pub fn from_scale(scale: Vec2) -> Mat2x3 {
        Mat2x3 {
            m00: scale.x,
            m01: 0.0,
            m10: 0.0,
            m11: scale.y,
            m20: 0.0,
            m21: 0.0,
        }
    }

    pub fn from_translate(translation: Vec2) -> Mat2x3 {
        Mat2x3 {
            m00: 1.0,
            m01: 0.0,
            m10: 0.0,
            m11: 1.0,
            m20: translation.x,
            m21: translation.y,
        }
    }

    pub fn from_reflect_origin() -> Mat2x3 {
        Mat2x3 {
            m00: -1.0,
            m01: 0.0,
            m10: 0.0,
            m11: -1.0,
            m20: 0.0,
            m21: 0.0,
        }
    }

    pub fn from_reflect_x() -> Mat2x3 {
        Mat2x3 {
            m00: 1.0,
            m01: 0.0,
            m10: 0.0,
            m11: -1.0,
            m20: 0.0,
            m21: 0.0,
        }
    }

    pub fn from_reflect_y() -> Mat2x3 {
        Mat2x3 {
            m00: -1.0,
            m01: 0.0,
            m10: 0.0,
            m11: 1.0,
            m20: 0.0,
            m21: 0.0,
        }
    }
}

macro_rules! from_to_self_and_copy {
    ($from_method:ident $(($($param:ident : $type:ty),* ))?, $method:ident, $copy_method:ident ) => {
        impl Mat2x3 {
            pub fn $method(&mut self $(, $($param : $type),* )?) -> Self {
                *self = Mat2x3::$from_method($( $($param),* )?) * *self;
                *self
            }

            pub fn $copy_method(&self $(, $($param : $type),* )?) -> Mat2x3 {
                Mat2x3::$from_method($( $($param),* )?) * *self
            }
        }
    };
}

from_to_self_and_copy!(from_rotation (angle: f32), rotate, rotate_copy);
from_to_self_and_copy!(from_scale (scale: Vec2), scale, scale_copy);
from_to_self_and_copy!(from_translate (translation: Vec2), translate, translate_copy);
from_to_self_and_copy!(from_reflect_origin (), reflect_origin, reflect_origin_copy);
from_to_self_and_copy!(from_reflect_x (), reflect_x, reflect_x_copy);
from_to_self_and_copy!(from_reflect_y (), reflect_y, reflect_y_copy);

impl Mul<Mat2x3> for Mat2x3 {
    type Output = Mat2x3;

    fn mul(self, rhs: Mat2x3) -> Mat2x3 {
        Mat2x3 {
            m00: self.m00 * rhs.m00 + self.m10 * rhs.m01,
            m01: self.m01 * rhs.m00 + self.m11 * rhs.m01,
            m10: self.m00 * rhs.m10 + self.m10 * rhs.m11,
            m11: self.m01 * rhs.m10 + self.m11 * rhs.m11,
            m20: self.m00 * rhs.m20 + self.m10 * rhs.m21 + self.m20,
            m21: self.m01 * rhs.m20 + self.m11 * rhs.m21 + self.m21,
        }
    }
}

forward_ref_binop!(impl Mul, mul for Mat2x3, Mat2x3);

impl Mul<Vec2> for Mat2x3 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.m00 * rhs.x + self.m01 * rhs.y + self.m20,
            y: self.m10 * rhs.x + self.m11 * rhs.y + self.m21,
        }
    }
}

forward_ref_binop!(impl Mul, mul for Mat2x3, Vec2);

#[cfg(test)]
mod test {
    use crate::pures::{Mat2x3, Vec2};

    #[test]
    fn test_identity() {
        let identity = Mat2x3::identity();
        let vec = Vec2::new(1.0, 1.0);
        assert_eq!(identity * vec, vec);
    }

    #[test]
    fn test_inverse() {
        let mat = Mat2x3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        println!("{}", mat);
        let inv = mat.inverse();
        println!("{}", inv);
        assert_eq!(mat * inv, Mat2x3::identity());
    }

    #[test]
    fn test_rotation() {
        let mat = Mat2x3::from_rotation(std::f32::consts::PI / 2.0);
        let vec = Vec2::new(1.0, 0.0);
        let rotated = mat * vec;
        assert!((rotated.x - 0.0).abs() < f32::EPSILON);
        assert!((rotated.y - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_scale() {
        let scale = Vec2::new(2.0, 3.0);
        let scaled = Mat2x3::from_scale(scale);

        let vec = Vec2::new(1.0, 1.0);
        let result = scaled * vec;
        assert_eq!(result, Vec2::new(2.0, 3.0));
    }

    #[test]
    fn test_translate() {
        let translation = Vec2::new(2.0, 3.0);
        let translated = Mat2x3::from_translate(translation);
        let vec = Vec2::new(1.0, 1.0);
        let result = translated * vec;
        assert_eq!(result, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_reflect_origin() {
        let reflect = Mat2x3::from_reflect_origin();
        let vec = Vec2::new(1.0, 1.0);
        let result = reflect * vec;
        assert_eq!(result, Vec2::new(-1.0, -1.0));
    }

    #[test]
    fn test_reflect_x() {
        let reflect = Mat2x3::from_reflect_x();
        let vec = Vec2::new(1.0, 1.0);
        let result = reflect * vec;
        assert_eq!(result, Vec2::new(1.0, -1.0));
    }

    #[test]
    fn test_reflect_y() {
        let reflect = Mat2x3::from_reflect_y();
        let vec = Vec2::new(1.0, 1.0);
        let result = reflect * vec;
        assert_eq!(result, Vec2::new(-1.0, 1.0));
    }
}
