use common::math::lerp;
use common::pures::Affine;
use common::types::{Coord, Length2d, Rect, ScreenCoord, ScreenLength2d, ScreenRect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct CameraSettings {
    pub pixel_region: ScreenRect,

    pub zoom_slope: f32,
    pub min_scaling_step: i32,
    pub max_scaling_step: i32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            pixel_region: ScreenRect::new(0.0, 0.0, 0.0, 0.0),
            zoom_slope: 1.1,
            min_scaling_step: 35,
            max_scaling_step: 50,
        }
    }
}

impl CameraSettings {
    pub fn min_scaling(&self) -> f32 {
        1.0 / (self.zoom_slope.powi(self.min_scaling_step))
    }

    pub fn max_scaling(&self) -> f32 {
        1.0 * (self.zoom_slope.powi(self.max_scaling_step))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Camera {
    position: Coord,
    scaling: f32,
    rotation: f32,
    reflect_x: bool,
    reflect_y: bool,
    base_scale: ScreenLength2d,

    #[serde(skip)]
    home: Coord,

    #[serde(skip)]
    pub settings: CameraSettings,
}

impl Default for Camera {
    fn default() -> Self {
        let default_translate = Coord::new(0.0, 0.0);

        Self {
            position: default_translate,
            scaling: 1.0,
            rotation: 0.0,
            reflect_x: false,
            reflect_y: false,
            base_scale: ScreenLength2d::new(500.0, 500.0),

            home: default_translate,
            settings: CameraSettings::default(),
        }
    }
}

impl Camera {
    pub fn new(default_translate: Coord, width: f32, height: f32) -> Self {
        Self {
            position: default_translate,
            scaling: 1.0,
            rotation: 0.0, //f32::to_radians(45.0),
            reflect_x: false,
            reflect_y: false,
            base_scale: ScreenLength2d::new(width, height),

            home: default_translate,
            settings: CameraSettings::default(),
        }
    }

    pub fn get_zoom(&self) -> f32 {
        self.scaling
    }

    pub fn get_pixel_region(&self) -> ScreenRect {
        self.settings.pixel_region
    }

    pub fn set_pixel_region(&mut self, width: f32, height: f32) {
        self.settings.pixel_region = ScreenRect::new(0.0, 0.0, width, height);
    }

    pub fn get_base_scale(&self) -> ScreenLength2d {
        self.base_scale
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_reflect_x(&mut self, reflect_x: bool) {
        self.reflect_x = reflect_x;
    }

    pub fn get_reflect_x(&self) -> bool {
        self.reflect_x
    }

    pub fn set_reflect_y(&mut self, reflect_y: bool) {
        self.reflect_y = reflect_y;
    }

    pub fn get_reflect_y(&self) -> bool {
        self.reflect_y
    }

    pub fn region(&self) -> Rect {
        let length = self.transform_to_length2d(self.settings.pixel_region.length());

        let pos = self.position - Coord::from(length) / 2.0;

        Rect::new(pos.x, pos.y, pos.x + length.x, pos.y + length.y)
    }

    /// Return the canvas coordinates of a given pixel point of the apps window.
    /// (0,0) is the top left corner of the window.
    pub fn project(&self, position: ScreenCoord) -> Coord {
        let result = self.get_inverse_transform() * position;

        Coord::from(result)
    }

    pub fn unproject(&self, position: Coord) -> ScreenCoord {
        let result = self.get_transform() * position;

        ScreenCoord::from(result)
    }

    pub fn unproject_to_canvas(&self, position: Coord) -> ScreenCoord {
        let m = Affine::identity()
            .translate(Coord::new(1.0, 1.0))
            .scale(self.base_scale / 2.0);
        let result = m * position;

        ScreenCoord::from(result)
    }

    pub fn transform_to_length2d(&self, movement: ScreenLength2d) -> Length2d {
        let res = movement / self.scaling;
        let res = Length2d::new(
            res.x / self.get_base_scale().x / 0.5,
            res.y / self.get_base_scale().y / 0.5,
        );

        res
    }

    pub fn transform_to_length2d_no_scale(&self, length: ScreenLength2d) -> Length2d {
        let res = Length2d::new(
            length.x / self.get_base_scale().x / 0.5,
            length.y / self.get_base_scale().y / 0.5,
        );

        res
    }

    pub fn transform_to_length2d_with_rotation(&self, movement: ScreenLength2d) -> Length2d {
        let m = self.get_inverse_transform();
        let result = m * movement - m * ScreenLength2d::new(0.0, 0.0);
        return Length2d::from(result);
    }

    ///  Zooms the camera in or out by the given amount, centered on the given point.
    ///
    ///  # Arguments
    ///  * `movement` - positive for zoom in, negative for zoom out

    pub fn zoom_at(&mut self, movement: f32, coord: ScreenCoord) {
        if movement < 0.0 && self.scaling >= self.settings.min_scaling()
            || movement > 0.0 && self.scaling <= self.settings.max_scaling()
        {
            let old_scaling = self.scaling;

            let new_scaling = self.compute_zoom(movement);

            let projected_coord = self.project(coord);

            let factor = 1.0 - (old_scaling / new_scaling);

            self.position = lerp(self.position, projected_coord, factor);

            self.scaling = new_scaling;
        };
    }

    fn compute_zoom(&self, movement: f32) -> f32 {
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

    pub fn pan_by(&mut self, movement: ScreenLength2d) {
        let movement = self.transform_to_length2d_with_rotation(movement);

        self.position = self.position - Coord::from(movement);
    }

    pub fn home(&mut self) {
        self.position = self.home;
        self.scaling = 1.0;
        self.rotation = 0.0;
        self.reflect_x = false;
        self.reflect_y = false;
    }
}

impl Camera {
    pub fn get_transform(&self) -> Affine {
        let translate_center = self.settings.pixel_region.center();

        let m_rot = Affine::identity()
            .translate(translate_center * -1.0)
            .rotate(-self.rotation)
            .translate(translate_center);

        let m_scale = Affine::identity()
            .scale(Length2d::new(0.5, 0.5))
            .scale(Length2d::new(self.scaling, self.scaling))
            .scale(self.get_base_scale());

        let m_translate = Affine::from_translate(self.region().top_left * -1.0);

        let mut rtn = m_rot * m_scale * m_translate;

        if self.reflect_x {
            let reflect_x = Affine::identity()
                .translate(translate_center * -1.0)
                .reflect_x()
                .translate(translate_center);
            rtn = reflect_x * rtn;
        }
        if self.reflect_y {
            let reflect_y = Affine::identity()
                .translate(translate_center * -1.0)
                .reflect_y()
                .translate(translate_center);
            rtn = reflect_y * rtn;
        }

        rtn
    }

    pub fn get_inverse_transform(&self) -> Affine {
        self.get_transform().inverse()
    }
}

#[cfg(test)]
mod test {
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn given_default_then_region() {
        let mut camera = Camera::default();
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        let region = camera.region();

        assert_approx_eq!(f32, region.top_left.x, -2.0);
        assert_approx_eq!(f32, region.top_left.y, -2.0);
        assert_approx_eq!(f32, region.bottom_right.x, 2.0);
        assert_approx_eq!(f32, region.bottom_right.y, 2.0);
    }

    #[test]
    fn when_zoom_center_then_region() {
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 2.0;
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.zoom_at(1.0, ScreenCoord::new(500.0, 500.0));

        let region = camera.region();

        assert_approx_eq!(f32, region.top_left.x, -1.0);
        assert_approx_eq!(f32, region.top_left.y, -1.0);
        assert_approx_eq!(f32, region.bottom_right.x, 1.0);
        assert_approx_eq!(f32, region.bottom_right.y, 1.0);
    }

    #[test]
    fn when_zoom_top_left_corner_then_region() {
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 1.5;
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);
        camera.zoom_at(1.0, ScreenCoord::new(250.0, 250.0));

        let region = camera.region();

        assert_approx_eq!(f32, camera.scaling, 1.5);
        assert_approx_eq!(f32, region.top_left.x, -1.66666666);
        assert_approx_eq!(f32, region.top_left.y, -1.66666666);
        assert_approx_eq!(f32, region.bottom_right.x, 1.0);
        assert_approx_eq!(f32, region.bottom_right.y, 1.0);
    }

    #[test]
    fn given_default_then_transform() {
        let mut camera = Camera::default();

        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        let transform = camera.get_transform();

        assert_approx_eq!(
            ScreenCoord,
            camera.unproject(Coord::new(-1.0, -1.0)),
            ScreenCoord::new(250.0, 250.0)
        );
        assert_approx_eq!(
            ScreenCoord,
            camera.unproject(Coord::new(1.0, 1.0)),
            ScreenCoord::new(750.0, 750.0)
        );

        assert_approx_eq!(f32, transform.get_translation().x, 500.0);
        assert_approx_eq!(f32, transform.get_translation().y, 500.0);
        assert_approx_eq!(f32, transform.get_scale().x, 250.0);
        assert_approx_eq!(f32, transform.get_scale().y, 250.0);
    }

    #[test]
    fn given_smaller_size_then_transform() {
        let mut camera = Camera::default();

        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 250.0, 250.0);

        let transform = camera.get_transform();

        assert_approx_eq!(f32, transform.get_translation().x, 125.0);
        assert_approx_eq!(f32, transform.get_translation().y, 125.0);
        assert_approx_eq!(f32, transform.get_scale().x, 250.0);
        assert_approx_eq!(f32, transform.get_scale().y, 250.0);
    }

    #[test]
    fn given_default_when_zoom_center_then_transform() {
        let mut camera = Camera::default();

        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);
        camera.settings.zoom_slope = 1.5;

        camera.zoom_at(1.0, ScreenCoord::new(500.0, 500.0));

        let transform = camera.get_transform();

        assert_approx_eq!(f32, transform.get_translation().x, 500.0);
        assert_approx_eq!(f32, transform.get_translation().y, 500.0);
        assert_approx_eq!(f32, transform.get_scale().x, 375.0);
        assert_approx_eq!(f32, transform.get_scale().y, 375.0);
    }

    #[test]
    fn when_zoom_top_left_then_transform() {
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 1.5;

        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);
        camera.zoom_at(1.0, ScreenCoord::new(250.0, 250.0));

        let transform = camera.get_transform();

        assert_approx_eq!(f32, camera.scaling, 1.5);
        assert_approx_eq!(f32, transform.get_translation().x, 625.0);
        assert_approx_eq!(f32, transform.get_translation().y, 625.0);
        assert_approx_eq!(f32, transform.get_scale().x, 375.0);
        assert_approx_eq!(f32, transform.get_scale().y, 375.0);
    }

    #[test]
    fn when_zoom_top_left_multiple_then_transform() {
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 1.25;
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.zoom_at(1.0, ScreenCoord::new(250.0, 250.0));
        camera.zoom_at(1.0, ScreenCoord::new(250.0, 250.0));

        let transform = camera.get_transform();

        assert_approx_eq!(f32, camera.scaling, 1.6);
        assert_approx_eq!(f32, transform.get_translation().x, 650.0);
        assert_approx_eq!(f32, transform.get_translation().y, 650.0);
        assert_approx_eq!(f32, transform.get_scale().x, 400.0);
        assert_approx_eq!(f32, transform.get_scale().y, 400.0);
    }

    #[test]
    fn when_zoom_bottom_right_2times_then_transform() {
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 1.25;
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));

        let transform = camera.get_transform();

        assert_approx_eq!(f32, camera.scaling, 1.6);
        assert_approx_eq!(f32, transform.get_translation().x, 350.0);
        assert_approx_eq!(f32, transform.get_translation().y, 350.0);
        assert_approx_eq!(f32, transform.get_scale().x, 400.0);
        assert_approx_eq!(f32, transform.get_scale().y, 400.0);
    }

    #[test]
    fn when_zoom_in_out_center_then_transform_same_then_start() {
        let mut camera = Camera::default();
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(-1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(-1.0, ScreenCoord::new(750.0, 750.0));

        let transform = camera.get_transform();

        assert_approx_eq!(f32, camera.scaling, 1.00);
        assert_approx_eq!(f32, transform.get_translation().x, 500.0);
        assert_approx_eq!(f32, transform.get_translation().y, 500.0);
        assert_approx_eq!(f32, transform.get_scale().x, 250.0);
        assert_approx_eq!(f32, transform.get_scale().y, 250.0);
    }

    #[test]
    fn when_zoom_bottom_right_multiple_then_transform() {
        let mut camera = Camera::default();
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));

        let transform = camera.get_transform();

        let minus = (500.0 * camera.scaling / 2.0) - 500.0;
        assert_approx_eq!(f32, camera.scaling, 3.4);
        assert_approx_eq!(f32, transform.get_translation().x, 250.0 - minus);
        assert_approx_eq!(f32, transform.get_translation().y, 250.0 - minus);
        assert_approx_eq!(f32, transform.get_scale().x, 500.0 * camera.scaling / 2.0);
        assert_approx_eq!(f32, transform.get_scale().y, 500.0 * camera.scaling / 2.0);
    }

    #[test]
    fn when_zoom_top_left_corner_then_project() {
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 1.5;
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);
        camera.zoom_at(1.0, ScreenCoord::new(250.0, 250.0));

        let coord = camera.project(ScreenCoord::new(0.0, 0.0));

        assert_approx_eq!(f32, coord.x, -1.666666666);
        assert_approx_eq!(f32, coord.y, -1.666666666);
    }

    #[test]
    fn when_rotation_90deg_then_unproject() {
        let mut camera = Camera::default();
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        let coord = camera.unproject(Coord::new(-1.0, -1.0));
        assert_approx_eq!(ScreenCoord, coord, ScreenCoord::new(250.0, 250.0));

        let coord = camera.unproject(Coord::new(1.0, -1.0));
        assert_approx_eq!(ScreenCoord, coord, ScreenCoord::new(750.0, 250.0));

        camera.set_rotation(f32::to_radians(90.0));

        let coord = camera.unproject(Coord::new(-1.0, -1.0));
        assert_approx_eq!(ScreenCoord, coord, ScreenCoord::new(250.0, 750.0));

        let coord = camera.unproject(Coord::new(1.0, -1.0));
        assert_approx_eq!(ScreenCoord, coord, ScreenCoord::new(250.0, 250.0));
    }
}
