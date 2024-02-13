use common::math::lerp;
use common::types::{Coord, Rect, ScreenCoord, ScreenRect};

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Coord,
    pub scaling: f32,
    home: Coord,
    pub pixel_region: ScreenRect,
    pub const_zoom: f32,
}

impl Camera {
    pub const MIN_SCALING: f32 = 0.1;
    pub const MAX_SCALING: f32 = 50.0;
    pub const WIDTH: f32 = 500.0;

    pub fn new() -> Self {
        let default_translate = Coord::new(0.0, 0.0);

        Self {
            position: default_translate,
            scaling: 1.0,
            home: default_translate,
            pixel_region: ScreenRect::new(0.0, 0.0, 0.0, 0.0),
            const_zoom: 15.0,
        }
    }

    pub fn new_center(default_translate: Coord) -> Self {
        Self {
            position: default_translate,
            scaling: 1.0,
            home: default_translate,
            pixel_region: ScreenRect::new(0.0, 0.0, 0.0, 0.0),
            const_zoom: 15.0,
        }
    }

    pub fn region(&self) -> Rect {
        let width = self.pixel_region.width() / self.scaling / Self::WIDTH;
        let height = self.pixel_region.height() / self.scaling / (Self::WIDTH);

        let x = self.position.x() - (width / 2.0);
        let y = self.position.x() - (height / 2.0);

        Rect::new(x, y, width, height)
    }

    /// Return the canvas coordinates of a given pixel point of the apps window.
    /// (0,0) is the top left corner of the window.
    ///
    pub fn project(&self, position: &ScreenCoord) -> Coord {
        let region = &self.region();

        let result = ((position.c - self.pixel_region.top_left.c) / self.scaling / Self::WIDTH)
            + region.top_left.c;

        Coord { c: result }
    }

    pub fn unproject(&self, position: &Coord) -> ScreenCoord {
        let region = &self.region();

        let result = ((position.c - region.top_left.c) * self.scaling * Self::WIDTH)
            + self.pixel_region.top_left.c;

        ScreenCoord { c: result }
    }

    /* pub fn project_in_view(&self, position: (f32, f32)) -> Option<(f32, f32)> {
        let point = self.project(position);

        match contain(self.region(), point) {
            true => Some(point),
            false => None,
        }
    }

    pub fn project_in_canvas(&self, position: (f32, f32)) -> Option<(f32, f32)> {
        let point = self.project(position);

        let canvas = (0.0, 0.0, 1.0, 1.0);

        match contain(canvas, point) {
            true => Some(point),
            false => None,
        }
    } */

    pub fn handle_zoom(&mut self, movement: f32, coord: ScreenCoord) {
        if movement < 0.0 && self.scaling > Self::MIN_SCALING
            || movement > 0.0 && self.scaling < Self::MAX_SCALING
        {
            let movement = f32::signum(movement);
            let old_scaling = self.scaling;

            let new_scaling = (self.scaling * (1.0 + movement / self.const_zoom))
                .clamp(Self::MIN_SCALING, Self::MAX_SCALING);

            let projected_coord = self.project(&coord);

            let factor = 1.0 - (old_scaling / new_scaling);

            self.position = Coord {
                c: lerp(&self.position.c, &projected_coord.c, factor),
            };

            self.scaling = new_scaling;
        };
    }

    pub fn handle_pan(&mut self, movement: (f32, f32)) {
        let scale_x = movement.0 / (Self::WIDTH * self.scaling);
        let scale_y = movement.1 / (Self::WIDTH * self.scaling);

        self.position = Coord::new(self.position.c.x + scale_x, self.position.c.y + scale_y)
    }

    pub fn get_transform(&self) -> (f32, f32, f32, f32) {
        let top_left_on_screen = self.unproject(&Coord::new(0.0, 0.0));

        let vgc_width = Self::WIDTH * self.scaling;
        let vgc_height = vgc_width;

        return (
            top_left_on_screen.c.x as f32,
            top_left_on_screen.c.y as f32,
            vgc_width as f32,
            vgc_height as f32,
        );
    }

    pub fn fixed_2d_length(&self, movement: (f32, f32)) -> (f32, f32) {
        let transform = self.get_transform();

        let x = movement.0 / transform.2;
        let y = movement.1 / transform.3;

        (x, y)
    }

    /// Return the length of a given fixed pixel length in the canvas.
    pub fn fixed_length(&self, length_px: f32) -> f32 {
        length_px / self.scaling / Self::WIDTH
    }

    pub fn home(&mut self) {
        self.position = self.home;
        self.scaling = 1.0;
    }

    pub fn handle_btn_zoom(&mut self, zoom: f32) {
        if zoom > 0.0 {
            self.scaling = (self.scaling * 1.1).clamp(Self::MIN_SCALING, Self::MAX_SCALING)
        } else {
            self.scaling = (self.scaling / 1.1).clamp(Self::MIN_SCALING, Self::MAX_SCALING)
        }
    }
}

#[cfg(test)]
mod test {
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn test_transform() {
        let mut camera = Camera::new();

        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        let transform = camera.get_transform();

        assert_approx_eq!(f32, transform.0, 250.0);
        assert_approx_eq!(f32, transform.1, 250.0);
        assert_approx_eq!(f32, transform.2, 500.0);
        assert_approx_eq!(f32, transform.3, 500.0);
    }

    #[test]
    fn change_size_transform() {
        let mut camera = Camera::new();

        camera.pixel_region = ScreenRect::new(0.0, 0.0, 250.0, 250.0);

        let transform = camera.get_transform();

        assert_approx_eq!(f32, transform.0, -125.0);
        assert_approx_eq!(f32, transform.1, -125.0);
        assert_approx_eq!(f32, transform.2, 500.0);
        assert_approx_eq!(f32, transform.3, 500.0);
    }

    #[test]
    fn test_zoom_in_center_then_transform() {
        let mut camera = Camera::new();
        camera.const_zoom = 2.0;

        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.handle_zoom(1.0, ScreenCoord::new(500.0, 500.0));
        camera.handle_zoom(1.0, ScreenCoord::new(500.0, 500.0));
        camera.handle_zoom(1.0, ScreenCoord::new(500.0, 500.0));
        camera.handle_zoom(1.0, ScreenCoord::new(500.0, 500.0));

        let transform = camera.get_transform();

        let minus = ((500.0 * camera.scaling) - 500.0) / 2.0;

        assert_eq!(transform.0, 250.0 - minus);
        assert_eq!(transform.1, 250.0 - minus);
        assert_eq!(transform.2, 500.0 * camera.scaling);
        assert_eq!(transform.3, 500.0 * camera.scaling);
    }

    #[test]
    fn zoom_in_corner_top_left_then_transform() {
        let mut camera = Camera::new();
        camera.const_zoom = 2.0;
        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);
        camera.handle_zoom(1.0, ScreenCoord::new(250.0, 250.0));

        let transform = camera.get_transform();

        assert_approx_eq!(f32, camera.scaling, 1.5);
        assert_approx_eq!(f32, transform.0, 250.0);
        assert_approx_eq!(f32, transform.1, 250.0);
        assert_approx_eq!(f32, transform.2, 500.0 * camera.scaling);
        assert_approx_eq!(f32, transform.3, 500.0 * camera.scaling);
    }

    #[test]
    fn test_no_zoom_then_region() {
        let mut camera = Camera::new();
        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        let region = camera.region();

        assert_approx_eq!(f32, region.bottom_right.c.x, -0.5);
        assert_approx_eq!(f32, region.bottom_right.c.y, -0.5);
        assert_approx_eq!(f32, region.top_left.c.x, 2.0);
        assert_approx_eq!(f32, region.top_left.c.y, 2.0);
    }

    #[test]
    fn test_zoom_multiple_in_corner_then_transform() {
        let mut camera = Camera::new();
        camera.const_zoom = 4.0;
        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.handle_zoom(1.0, ScreenCoord::new(250.0, 250.0));
        camera.handle_zoom(1.0, ScreenCoord::new(250.0, 250.0));

        let transform = camera.get_transform();

        assert_approx_eq!(f32, camera.scaling, 1.5625);
        assert_approx_eq!(f32, transform.0, 250.0);
        assert_approx_eq!(f32, transform.1, 250.0);
        assert_approx_eq!(f32, transform.2, 500.0 * camera.scaling);
        assert_approx_eq!(f32, transform.3, 500.0 * camera.scaling);
    }

    #[test]
    fn test_zoom_multiple_in_corner_fast_right_then_transform() {
        let mut camera = Camera::new();
        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);
        camera.const_zoom = 4.0;

        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));

        let transform = camera.get_transform();

        let minus = (500.0 * camera.scaling) - 500.0;
        assert_approx_eq!(f32, camera.scaling, 1.5625);
        assert_approx_eq!(f32, transform.0, 250.0 - minus, (0.0001, 3));
        assert_approx_eq!(f32, transform.1, 250.0 - minus, (0.0001, 3));
        assert_approx_eq!(f32, transform.2, 500.0 * camera.scaling, (0.0001, 3));
        assert_approx_eq!(f32, transform.3, 500.0 * camera.scaling, (0.0001, 3));
    }

    #[test]
    fn zoom_in_out_then_transform() {
        let mut camera = Camera::new();
        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);
        camera.const_zoom = 4.0;

        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(-1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(-1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));

        let transform = camera.get_transform();

        assert_approx_eq!(f32, camera.scaling, 1.0986328);
        assert_approx_eq!(f32, transform.0, 250.0, (0.0001, 3));
        assert_approx_eq!(f32, transform.1, 250.0, (0.0001, 3));
        assert_approx_eq!(f32, transform.2, 500.0, (0.0001, 3));
        assert_approx_eq!(f32, transform.3, 500.0, (0.0001, 3));
    }

    #[test]
    fn test_zoom_multiple_in_corner_right_then_transform() {
        let mut camera = Camera::new();
        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));

        let transform = camera.get_transform();

        let minus = (500.0 * camera.scaling) - 500.0;
        assert_approx_eq!(f32, camera.scaling, 1.5315307, (0.0001, 3));
        assert_approx_eq!(f32, transform.0, 250.0 - minus, (0.0001, 3));
        assert_approx_eq!(f32, transform.1, 250.0 - minus, (0.0001, 3));
        assert_approx_eq!(f32, transform.2, 500.0 * camera.scaling, (0.0001, 3));
        assert_approx_eq!(f32, transform.3, 500.0 * camera.scaling, (0.0001, 3));
    }

    #[test]
    fn test_zoom_center_then_region() {
        let mut camera = Camera::new();
        camera.scaling = 2.0;
        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        let region = camera.region();

        assert_approx_eq!(f32, region.bottom_right.c.x, 0.0);
        assert_approx_eq!(f32, region.bottom_right.c.y, 0.0);
        assert_approx_eq!(f32, region.top_left.c.x, 1.0);
        assert_approx_eq!(f32, region.top_left.c.y, 1.0);
    }

    #[test]
    fn test_zoom_top_left_corner_then_region() {
        let mut camera = Camera::new();
        camera.const_zoom = 2.0;
        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);
        camera.handle_zoom(1.0, ScreenCoord::new(250.0, 250.0));

        let region = camera.region();

        assert_approx_eq!(f32, camera.scaling, 1.5);
        assert_approx_eq!(f32, region.bottom_right.c.x, -0.333333333);
        assert_approx_eq!(f32, region.bottom_right.c.y, -0.333333333);
        assert_approx_eq!(f32, region.top_left.c.x, 1.333333333);
        assert_approx_eq!(f32, region.top_left.c.y, 1.333333333);
    }
}
