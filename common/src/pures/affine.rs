use std::{fmt::Display, ops::Mul};

use float_cmp::{ApproxEq, F32Margin};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::forward_ref_binop;

use super::Vec2;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
// Transformation matrix for 2D space
pub struct Affine {
    /// m00 m01 m02
    /// m10 m11 m12
    ///  0   0   1
    m00: f32,
    m10: f32,
    m01: f32,
    m11: f32,
    m02: f32,
    m12: f32,
}

impl Display for Affine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{},{})\n({},{},{})",
            self.m00, self.m01, self.m02, self.m10, self.m11, self.m12
        )
    }
}

impl Affine {
    pub fn new(m00: f32, m10: f32, m01: f32, m11: f32, m02: f32, m12: f32) -> Affine {
        Affine {
            m00,
            m10,
            m01,
            m11,
            m02,
            m12,
        }
    }

    pub fn identity() -> Affine {
        Affine {
            m00: 1.0,
            m10: 0.0,
            m01: 0.0,
            m11: 1.0,
            m02: 0.0,
            m12: 0.0,
        }
    }

    pub fn inverse(&self) -> Affine {
        let inv_det = 1.0 / (self.m00 * self.m11 - self.m10 * self.m01);

        let new_m00 = self.m11 * inv_det;
        let new_m10 = -self.m10 * inv_det;
        let new_m01 = -self.m01 * inv_det;
        let new_m11 = self.m00 * inv_det;
        let new_m02 = -(self.m11 * self.m02 - self.m01 * self.m12) * inv_det;
        let new_m12 = (self.m10 * self.m02 - self.m00 * self.m12) * inv_det;

        Affine {
            m00: new_m00,
            m10: new_m10,
            m01: new_m01,
            m11: new_m11,
            m02: new_m02,
            m12: new_m12,
        }
    }

    pub fn get_scale(&self) -> Vec2 {
        Vec2::new(self.m00, self.m11)
    }

    pub fn get_translation(&self) -> Vec2 {
        Vec2::new(self.m02, self.m12)
    }

    pub fn from_rotation(angle: f32) -> Affine {
        let (s, c) = angle.sin_cos();
        Affine {
            m00: c,
            m10: s,
            m01: -s,
            m11: c,
            m02: 0.0,
            m12: 0.0,
        }
    }

    pub fn from_scale(scale: Vec2) -> Affine {
        Affine {
            m00: scale.x,
            m10: 0.0,
            m01: 0.0,
            m11: scale.y,
            m02: 0.0,
            m12: 0.0,
        }
    }

    pub fn from_translate(translation: Vec2) -> Affine {
        Affine {
            m00: 1.0,
            m10: 0.0,
            m01: 0.0,
            m11: 1.0,
            m02: translation.x,
            m12: translation.y,
        }
    }

    pub fn from_reflect_origin() -> Affine {
        Affine {
            m00: -1.0,
            m10: 0.0,
            m01: 0.0,
            m11: -1.0,
            m02: 0.0,
            m12: 0.0,
        }
    }

    pub fn from_reflect_x() -> Affine {
        Affine {
            m00: 1.0,
            m10: 0.0,
            m01: 0.0,
            m11: -1.0,
            m02: 0.0,
            m12: 0.0,
        }
    }

    pub fn from_reflect_y() -> Affine {
        Affine {
            m00: -1.0,
            m10: 0.0,
            m01: 0.0,
            m11: 1.0,
            m02: 0.0,
            m12: 0.0,
        }
    }
}

macro_rules! from_to_self_and_copy {
    ($from_method:ident $(($($param:ident : $type:ty),* ))?, $method:ident, $copy_method:ident ) => {
        impl Affine {
            pub fn $method(&mut self $(, $($param : $type),* )?) -> Self {
                *self = Affine::$from_method($( $($param),* )?) * *self;
                *self
            }

            pub fn $copy_method(&self $(, $($param : $type),* )?) -> Affine {
                Affine::$from_method($( $($param),* )?) * *self
            }
        }
    };
}

from_to_self_and_copy!(from_rotation(angle: f32), rotate, rotate_copy);
from_to_self_and_copy!(from_scale(scale: Vec2), scale, scale_copy);
from_to_self_and_copy!(from_translate(translation: Vec2), translate, translate_copy);
from_to_self_and_copy!(from_reflect_origin(), reflect_origin, reflect_origin_copy);
from_to_self_and_copy!(from_reflect_x(), reflect_x, reflect_x_copy);
from_to_self_and_copy!(from_reflect_y(), reflect_y, reflect_y_copy);

impl Mul<Affine> for Affine {
    type Output = Affine;

    fn mul(self, rhs: Affine) -> Affine {
        Affine {
            m00: self.m00 * rhs.m00 + self.m01 * rhs.m10,
            m10: self.m10 * rhs.m00 + self.m11 * rhs.m10,
            m01: self.m00 * rhs.m01 + self.m01 * rhs.m11,
            m11: self.m10 * rhs.m01 + self.m11 * rhs.m11,
            m02: self.m00 * rhs.m02 + self.m01 * rhs.m12 + self.m02,
            m12: self.m10 * rhs.m02 + self.m11 * rhs.m12 + self.m12,
        }
    }
}

forward_ref_binop!(impl Mul, mul for Affine, Affine);

impl Mul<Vec2> for Affine {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.m00 * rhs.x + self.m01 * rhs.y + self.m02,
            y: self.m10 * rhs.x + self.m11 * rhs.y + self.m12,
        }
    }
}

forward_ref_binop!(impl Mul, mul for Affine, Vec2);

impl ApproxEq for Affine {
    type Margin = F32Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        self.m00.approx_eq(other.m00, margin)
            && self.m10.approx_eq(other.m10, margin)
            && self.m01.approx_eq(other.m01, margin)
            && self.m11.approx_eq(other.m11, margin)
            && self.m02.approx_eq(other.m02, margin)
            && self.m12.approx_eq(other.m12, margin)
    }
}

#[cfg(test)]
mod test {
    use float_cmp::assert_approx_eq;

    use crate::pures::{Affine, Vec2};

    #[test]
    fn test_identity() {
        let identity = Affine::identity();
        let vec = Vec2::new(1.0, 1.0);
        assert_eq!(identity * vec, vec);
    }

    #[test]
    fn test_inverse() {
        let mat = Affine::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);

        let inv = mat.inverse();

        assert_eq!(mat * inv, Affine::identity());
    }

    #[test]
    fn test_rotation() {
        let mat = Affine::from_rotation(std::f32::consts::PI / 2.0);
        let vec = Vec2::new(1.0, 0.0);
        let rotated = mat * vec;
        assert_approx_eq!(Vec2, rotated, Vec2::new(0.0, 1.0));
    }

    #[test]
    fn test_scale() {
        let scale = Vec2::new(2.0, 3.0);
        let scaled = Affine::from_scale(scale);

        let vec = Vec2::new(1.0, 1.0);
        let result = scaled * vec;
        assert_eq!(result, Vec2::new(2.0, 3.0));
    }

    #[test]
    fn test_translate() {
        let translation = Vec2::new(2.0, 3.0);
        let translated = Affine::from_translate(translation);
        let vec = Vec2::new(1.0, 1.0);
        let result = translated * vec;
        assert_eq!(result, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_reflect_origin() {
        let reflect = Affine::from_reflect_origin();
        let vec = Vec2::new(1.0, 1.0);
        let result = reflect * vec;
        assert_eq!(result, Vec2::new(-1.0, -1.0));
    }

    #[test]
    fn test_reflect_x() {
        let reflect = Affine::from_reflect_x();
        let vec = Vec2::new(1.0, 1.0);
        let result = reflect * vec;
        assert_eq!(result, Vec2::new(1.0, -1.0));
    }

    #[test]
    fn test_reflect_y() {
        let reflect = Affine::from_reflect_y();
        let vec = Vec2::new(1.0, 1.0);
        let result = reflect * vec;
        assert_eq!(result, Vec2::new(-1.0, 1.0));
    }

    #[test]
    fn rotation_of_square_at_center() {
        let mat = Affine::identity()
            .translate(Vec2::new(-0.5, -0.5))
            .rotate(std::f32::consts::PI / 4.0)
            .translate(Vec2::new(0.5, 0.5));

        let m_res = Affine::new(
            0.70710677,
            0.70710677,
            -0.70710677,
            0.70710677,
            0.5,
            -0.20710677,
        );

        assert_approx_eq!(Affine, mat, m_res);

        let p00 = Vec2::new(0.0, 0.0);
        let p01 = Vec2::new(1.0, 0.0);
        let p10 = Vec2::new(0.0, 1.0);
        let p11 = Vec2::new(1.0, 1.0);

        let p00 = mat * p00;
        let p01 = mat * p01;
        let p10 = mat * p10;
        let p11 = mat * p11;

        assert_approx_eq!(Vec2, p00, Vec2::new(0.5, -0.20710677));
        assert_approx_eq!(Vec2, p01, Vec2::new(1.20710677, 0.5));
        assert_approx_eq!(Vec2, p10, Vec2::new(-0.20710677, 0.5));
        assert_approx_eq!(Vec2, p11, Vec2::new(0.5, 1.20710677));
    }
}
