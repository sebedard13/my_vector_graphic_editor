use common::math::lerp;
use common::types::{
    Coord, Length, Length2d, Rect, ScreenCoord, ScreenLength, ScreenLength2d, ScreenRect,
};

#[derive(Debug, Clone)]
pub struct CameraSettings {
    pub base_width: ScreenLength,

    pub zoom_slope: f32,
    pub min_scaling_step: i32,
    pub max_scaling_step: i32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            zoom_slope: 1.1,
            base_width: ScreenLength::new(500.0),
            min_scaling_step: 35,
            max_scaling_step: 50,
        }
    }
}

impl CameraSettings {
    pub fn new(base_width: ScreenLength) -> Self {
        Self {
            base_width: base_width,
            ..Default::default()
        }
    }

    pub fn min_scaling(&self) -> f32 {
        1.0 / (self.zoom_slope.powi(self.min_scaling_step))
    }

    pub fn max_scaling(&self) -> f32 {
        1.0 * (self.zoom_slope.powi(self.max_scaling_step))
    }
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Coord,
    pub scaling: f32,
    home: Coord,
    pub pixel_region: ScreenRect,

    pub settings: CameraSettings,
}

impl Default for Camera {
    fn default() -> Self {
        let default_translate = Coord::new(0.5, 0.5);

        Self {
            position: default_translate,
            scaling: 1.0,
            home: default_translate,
            pixel_region: ScreenRect::new(0.0, 0.0, 0.0, 0.0),
            settings: CameraSettings::default(),
        }
    }
}

impl Camera {
    pub fn new_center(default_translate: Coord, width: f32) -> Self {
        Self {
            position: default_translate,
            scaling: 1.0,
            home: default_translate,
            pixel_region: ScreenRect::new(0.0, 0.0, 0.0, 0.0),
            settings: CameraSettings::new(ScreenLength::new(width)),
        }
    }

    pub fn region(&self) -> Rect {
        let width = self.pixel_region.width() / self.scaling / self.settings.base_width.c;
        let height = self.pixel_region.height() / self.scaling / (self.settings.base_width.c);

        let x = self.position.x() - (width / 2.0);
        let y = self.position.y() - (height / 2.0);

        Rect::new(x, y, x + width, y + height)
    }

    /// Return the canvas coordinates of a given pixel point of the apps window.
    /// (0,0) is the top left corner of the window.
    ///
    pub fn project(&self, position: &ScreenCoord) -> Coord {
        let region = &self.region();

        let result = ((position.c - self.pixel_region.top_left.c)
            / self.scaling
            / self.settings.base_width.c)
            + region.top_left.c;

        Coord { c: result }
    }

    pub fn unproject(&self, position: &Coord) -> ScreenCoord {
        let region = &self.region();

        let result = ((position.c - region.top_left.c) * self.scaling * self.settings.base_width.c)
            + self.pixel_region.top_left.c;

        ScreenCoord { c: result }
    }

    fn calcul_new_scaling(&self, movement: f32) -> f32 {
        let mut new_scaling = {
            if movement > 0.0 {
                (self.scaling * self.settings.zoom_slope)
                    .clamp(self.settings.min_scaling(), self.settings.max_scaling())
            } else {
                (self.scaling / self.settings.zoom_slope)
                    .clamp(self.settings.min_scaling(), self.settings.max_scaling())
            }
        };

        //Round to the nearest 0.1 or less significant digit
        if new_scaling > 0.5 {
            new_scaling = {
                let mut new_scaling = new_scaling * 10.0;
                new_scaling = new_scaling.round();
                new_scaling / 10.0
            };
        }

        new_scaling
    }

    pub fn handle_zoom(&mut self, movement: f32, coord: ScreenCoord) {
        if movement < 0.0 && self.scaling >= self.settings.min_scaling()
            || movement > 0.0 && self.scaling <= self.settings.max_scaling()
        {
            let old_scaling = self.scaling;

            let new_scaling = self.calcul_new_scaling(movement);

            let projected_coord = self.project(&coord);

            let factor = 1.0 - (old_scaling / new_scaling);

            self.position = Coord {
                c: lerp(&self.position.c, &projected_coord.c, factor),
            };

            self.scaling = new_scaling;
        };
    }

    pub fn handle_pan(&mut self, movement: ScreenLength2d) {
        let movement = self.fixed_2d_length(movement);

        self.position = Coord {
            c: self.position.c - movement.c,
        }
    }

    pub fn get_transform(&self) -> (f32, f32, f32, f32) {
        let top_left_on_screen = self.unproject(&Coord::new(0.0, 0.0));

        let vgc_width = self.settings.base_width.c * self.scaling;
        let vgc_height = vgc_width;

        return (
            top_left_on_screen.c.x as f32,
            top_left_on_screen.c.y as f32,
            vgc_width as f32,
            vgc_height as f32,
        );
    }

    pub fn fixed_2d_length(&self, movement: ScreenLength2d) -> Length2d {
        Length2d {
            c: movement.c / self.scaling / self.settings.base_width.c,
        }
    }

    /// Return the length of a given fixed pixel length in the canvas.
    pub fn fixed_length(&self, length: ScreenLength) -> Length {
        Length {
            c: length.c / self.scaling / self.settings.base_width.c,
        }
    }

    pub fn home(&mut self) {
        self.position = self.home;
        self.scaling = 1.0;
    }
}

#[cfg(test)]
mod test {
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn test_transform() {
        let mut camera = Camera::default();

        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        let transform = camera.get_transform();

        assert_approx_eq!(f32, transform.0, 250.0);
        assert_approx_eq!(f32, transform.1, 250.0);
        assert_approx_eq!(f32, transform.2, 500.0);
        assert_approx_eq!(f32, transform.3, 500.0);
    }

    #[test]
    fn change_size_transform() {
        let mut camera = Camera::default();

        camera.pixel_region = ScreenRect::new(0.0, 0.0, 250.0, 250.0);

        let transform = camera.get_transform();

        assert_approx_eq!(f32, transform.0, -125.0);
        assert_approx_eq!(f32, transform.1, -125.0);
        assert_approx_eq!(f32, transform.2, 500.0);
        assert_approx_eq!(f32, transform.3, 500.0);
    }

    #[test]
    fn test_zoom_in_center_then_transform() {
        let mut camera = Camera::default();

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
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 1.5;

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
    fn no_zoom_then_region() {
        let mut camera = Camera::default();
        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        let region = camera.region();

        assert_approx_eq!(f32, region.top_left.c.x, -0.5);
        assert_approx_eq!(f32, region.top_left.c.y, -0.5);
        assert_approx_eq!(f32, region.bottom_right.c.x, 1.5);
        assert_approx_eq!(f32, region.bottom_right.c.y, 1.5);
    }

    #[test]
    fn test_zoom_multiple_in_corner_then_transform() {
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 1.25;
        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.handle_zoom(1.0, ScreenCoord::new(250.0, 250.0));
        camera.handle_zoom(1.0, ScreenCoord::new(250.0, 250.0));

        let transform = camera.get_transform();

        assert_approx_eq!(f32, camera.scaling, 1.6);
        assert_approx_eq!(f32, transform.0, 250.0);
        assert_approx_eq!(f32, transform.1, 250.0);
        assert_approx_eq!(f32, transform.2, 500.0 * camera.scaling);
        assert_approx_eq!(f32, transform.3, 500.0 * camera.scaling);
    }

    #[test]
    fn test_zoom_multiple_in_corner_fast_right_then_transform() {
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 1.25;
        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));

        let transform = camera.get_transform();

        let minus = (500.0 * camera.scaling) - 500.0;
        assert_approx_eq!(f32, camera.scaling, 1.6);
        assert_approx_eq!(f32, transform.0, 250.0 - minus, (0.0001, 3));
        assert_approx_eq!(f32, transform.1, 250.0 - minus, (0.0001, 3));
        assert_approx_eq!(f32, transform.2, 500.0 * camera.scaling, (0.0001, 3));
        assert_approx_eq!(f32, transform.3, 500.0 * camera.scaling, (0.0001, 3));
    }

    #[test]
    fn given_camera_when_zoom_in_zoom_out_then_transform_same_then_start() {
        let mut camera = Camera::default();
        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(-1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(1.0, ScreenCoord::new(750.0, 750.0));
        camera.handle_zoom(-1.0, ScreenCoord::new(750.0, 750.0));

        let transform = camera.get_transform();

        assert_approx_eq!(f32, camera.scaling, 1.00);
        assert_approx_eq!(f32, transform.0, 250.0, (0.0001, 3));
        assert_approx_eq!(f32, transform.1, 250.0, (0.0001, 3));
        assert_approx_eq!(f32, transform.2, 500.0, (0.0001, 3));
        assert_approx_eq!(f32, transform.3, 500.0, (0.0001, 3));
    }

    #[test]
    fn test_zoom_multiple_in_corner_right_then_transform() {
        let mut camera = Camera::default();
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
        assert_approx_eq!(f32, camera.scaling, 3.4, (0.0001, 3));
        assert_approx_eq!(f32, transform.0, 250.0 - minus, (0.0001, 3));
        assert_approx_eq!(f32, transform.1, 250.0 - minus, (0.0001, 3));
        assert_approx_eq!(f32, transform.2, 500.0 * camera.scaling, (0.0001, 3));
        assert_approx_eq!(f32, transform.3, 500.0 * camera.scaling, (0.0001, 3));
    }

    #[test]
    fn test_zoom_center_then_region() {
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 2.0;
        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.handle_zoom(1.0, ScreenCoord::new(500.0, 500.0));

        let region = camera.region();

        assert_approx_eq!(f32, region.top_left.c.x, 0.0);
        assert_approx_eq!(f32, region.top_left.c.y, 0.0);
        assert_approx_eq!(f32, region.bottom_right.c.x, 1.0);
        assert_approx_eq!(f32, region.bottom_right.c.y, 1.0);
    }

    #[test]
    fn test_zoom_top_left_corner_then_region() {
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 1.5;
        camera.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);
        camera.handle_zoom(1.0, ScreenCoord::new(250.0, 250.0));

        let region = camera.region();

        assert_approx_eq!(f32, camera.scaling, 1.5);
        assert_approx_eq!(f32, region.top_left.c.x, -0.333333333);
        assert_approx_eq!(f32, region.top_left.c.y, -0.333333333);
        assert_approx_eq!(f32, region.bottom_right.c.x, 1.0);
        assert_approx_eq!(f32, region.bottom_right.c.y, 1.0);
    }
}
