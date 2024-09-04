use float_cmp::ApproxEq;

use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::PRECISION;

pub trait Vec2:
    Clone
    + Debug
    + PartialEq
    + Copy
    + Default
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<f32, Output = Self>
    + Div<f32, Output = Self>
    + Neg<Output = Self>
    + ApproxEq
{
    fn x(&self) -> f32;
    fn y(&self) -> f32;

    fn set_x(&mut self, value: f32);
    fn set_y(&mut self, value: f32);

    fn norm(&self) -> f32 {
        (self.x() * self.x() + self.y() * self.y()).sqrt()
    }

    fn normal(&self) -> Self {
        let norm = self.norm();
        let mut copy = self.clone();
        copy.set_y(copy.y() / norm);
        copy.set_x(copy.x() / norm);
        copy
    }

    fn normalize(&mut self) {
        let norm = self.norm();
        self.set_x(self.x() / norm);
        self.set_y(self.y() / norm);
    }

    fn abs(&mut self) {
        self.set_x(self.x().abs());
        self.set_y(self.y().abs());
    }

    fn distance(&self, other: &Self) -> f32 {
        let dx = self.x() - other.x();
        let dy = self.y() - other.y();
        (dx * dx + dy * dy).sqrt()
    }

    fn approx_distance(&self, other: &Self) -> f32 {
        let dx = self.x() - other.x();
        let dy = self.y() - other.y();
        dx * dx + dy * dy
    }

    fn min(a: &Self, b: &Self) -> Self {
        let mut copy = a.clone();
        copy.set_x(a.x().min(b.x()));
        copy.set_y(a.y().min(b.y()));
        copy
    }

    fn max(a: &Self, b: &Self) -> Self {
        let mut copy = a.clone();
        copy.set_x(a.x().max(b.x()));
        copy.set_y(a.y().max(b.y()));
        copy
    }

    fn vector_direction_equal(&self, other: &Self) -> bool {
        let dot = self.dot(other);
        let res = dot / (self.norm() * other.norm());
        (res.abs() - 1.0).abs() <= PRECISION
    }

    fn dot(&self, other: &Self) -> f32 {
        self.x() * other.x() + self.y() * other.y()
    }
}

#[macro_export]
macro_rules! vec2_op {
    ($type:ident) => {
        use std::ops::{Add, Div, Mul, Neg, Sub};
        use crate::{forward_ref_binop, forward_ref_unop};

        impl Add<$type> for $type {
            type Output = $type;

            fn add(self, other: Self) -> Self {
                $type {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        forward_ref_binop!(impl Add, add for $type, $type);

        impl Sub<$type> for $type {
            type Output = $type;

            fn sub(self, other: Self) -> Self {
                $type {
                    x: self.x - other.x,
                    y: self.y - other.y,
                }
            }
        }

        forward_ref_binop!(impl Sub, sub for $type, $type);

        impl Mul<f32> for $type {
            type Output = $type;

            fn mul(self, other: f32) -> $type {
                $type {
                    x: self.x * other,
                    y: self.y * other,
                }
            }
        }

        forward_ref_binop!(impl Mul, mul for $type, f32);

        impl Mul<$type> for f32 {
            type Output = $type;

            fn mul(self, other: $type) -> $type {
                $type {
                    x: self * other.x,
                    y: self * other.y,
                }
            }
        }

        forward_ref_binop!(impl Mul, mul for f32, $type);

        impl Div<f32> for $type {
            type Output = $type;

            fn div(self, other: f32) -> $type {
                $type {
                    x: self.x / other,
                    y: self.y / other,
                }
            }
        }

        forward_ref_binop!(impl Div, div for $type, f32);

        impl Mul<$type> for $type {
            type Output = $type;

            fn mul(self, other: $type) -> $type {
                $type {
                    x: self.x * other.x,
                    y: self.y * other.y,
                }
            }
        }

        forward_ref_binop!(impl Mul, mul for $type, $type);

        impl Neg for $type {
            type Output = $type;

            fn neg(self) -> $type {
                $type {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }

        forward_ref_unop!(impl Neg, neg for $type);
    };
}
