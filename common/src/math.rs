use super::pures::Vec2;

/// Return true if the cursor is in the radius of the center
///```rust
///
/// let cursor = Vec2::new(10.0, 10.0));
/// let center = Vec2::new(0.0, 0.0);
/// let radius = 5.0;
/// assert_eq!(point_in_radius(cursor, center, radius), false);
/// let cursor = Cursor::Available(Point::new(-3.0, 0.0));
/// assert_eq!(point_in_radius(cursor, center, radius), true);
///```
pub fn point_in_radius(point: &Vec2, center: &Vec2, radius: f32) -> bool {
    let x = point.x - center.x;
    let y = point.y - center.y;
    let distance = x * x + y * y;
    distance < (radius * radius)
}

pub fn contain(rect_min: &Vec2, rect_max: &Vec2, point: &Vec2) -> bool {
    point.x >= rect_min.x && point.x <= rect_max.x && point.y >= rect_min.y && point.y <= rect_max.y
}

pub fn lerp(a: &Vec2, b: &Vec2, t: f32) -> Vec2 {
    (1.0 - t) * a + t * b
}
