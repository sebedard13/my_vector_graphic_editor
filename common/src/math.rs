use super::pures::Vec2;

/// Return true if the cursor is in the radius of the center
///```rust
/// use common::math::point_in_radius;
/// use common::types::Coord;
///
/// let cursor = Coord::new(10.0, 10.0);
/// let center = Coord::new(0.0, 0.0);
/// let radius = Coord::new(5.0, 5.0);
/// assert_eq!(point_in_radius(cursor, center, radius), false);
/// let cursor = Coord::new(-3.0, 0.0);
/// assert_eq!(point_in_radius(cursor, center, radius), true);
///```
pub fn point_in_radius<T: Vec2>(point: T, center: T, radius: impl Vec2) -> bool {
    let mut value = point - center;
    value.set_x(value.x() / radius.x());
    value.set_y(value.y() / radius.y());
    value.x() * value.x() + value.y() * value.y() < 1.0
}

pub fn contain<T: Vec2>(rect_min: T, rect_max: T, point: T) -> bool {
    point.x() >= rect_min.x()
        && point.x() <= rect_max.x()
        && point.y() >= rect_min.y()
        && point.y() <= rect_max.y()
}

/// Linear interpolation between two vectors
/// ```rust
/// use common::types::Coord;
/// use common::math::lerp;
///
/// let a = Coord::new(0.0, 0.0);
/// let b = Coord::new(1.0, 2.0);
/// let t = 0.6;
/// assert_eq!(lerp(a, b, t), Coord::new(0.6, 1.2));
pub fn lerp<T: Vec2>(a: T, b: T, t: f32) -> T {
    a * (1.0 - t) + b * t
}
