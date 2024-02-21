use common::math::lerp;
use common::pures::{Mat2x3, Vec2};
use common::types::{
    Coord, Length, Length2d, Rect, ScreenCoord, ScreenLength, ScreenLength2d, ScreenRect,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::CanvasContent;

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
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

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    position: Coord,
    scaling: f32,
    rotation: f32,
    reflect_x: bool,
    reflect_y: bool,
    base_scale: ScreenLength,

    cache_transform: Mat2x3,
    home: Coord,
    pub settings: CameraSettings,
}

impl Default for Camera {
    fn default() -> Self {
        let default_translate = Coord::new(0.5, 0.5);

        Self {
            position: default_translate,
            scaling: 1.0,
            rotation: 0.0,
            reflect_x: false,
            reflect_y: false,
            base_scale: ScreenLength::new(500.0),

            cache_transform: Mat2x3::identity(),
            home: default_translate,
            settings: CameraSettings::default(),
        }
    }
}

#[macro_export]
macro_rules! generate_child_methods {
    ($parent:ident, $child:ident $(, ($method_par:ident, $method:ident $(($($param:ident : $type:ty),* ))?$(, $rtn:ty)?))+ ) => {
        #[wasm_bindgen]
        impl $parent {
            $(
                pub fn $method_par(&mut self $(, $($param : $type),* )?) $(-> $rtn)? {
                    self.$child.$method($( $($param),* )?)
                }
            )*
        }
    };
}

impl Camera {
    pub fn new(default_translate: Coord, width: f32) -> Self {
        Self {
            position: default_translate,
            scaling: 1.0,
            rotation: 0.0, //f32::to_radians(45.0),
            reflect_x: false,
            reflect_y: false,
            base_scale: ScreenLength::new(width),

            cache_transform: Mat2x3::identity(),
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

    pub fn get_base_scale(&self) -> ScreenLength {
        self.base_scale
    }

    pub fn region(&self) -> Rect {
        let length = self.transform_to_length2d(self.settings.pixel_region.length());

        let pos = self.position.c - length.c / 2.0;

        Rect::new(pos.x, pos.y, pos.x + length.c.x, pos.y + length.c.y)
    }

    /// Return the canvas coordinates of a given pixel point of the apps window.
    /// (0,0) is the top left corner of the window.
    pub fn project(&self, position: ScreenCoord) -> Coord {
        let result = self.get_inverse_transform() * position.c;

        Coord { c: result }
    }

    pub fn unproject(&self, position: Coord) -> ScreenCoord {
        let result = self.get_transform() * position.c;

        ScreenCoord { c: result }
    }

    pub fn transform_to_length2d(&self, movement: ScreenLength2d) -> Length2d {
        Length2d {
            c: movement.c / self.scaling / self.get_base_scale().c,
        }
    }

    /// Return the length of a given fixed pixel length in the canvas.
    pub fn transform_to_length(&self, length: ScreenLength) -> Length {
        Length {
            c: length.c / self.scaling / self.get_base_scale().c,
        }
    }

    pub fn transform_to_length2d_with_rotation(&self, movement: ScreenLength2d) -> Length2d {
        let m = self.get_inverse_transform();
        let result = m * movement.c - m * Vec2::new(0.0, 0.0);
        return Length2d { c: result };
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

            self.position = Coord {
                c: lerp(&self.position.c, &projected_coord.c, factor),
            };

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

        self.position = Coord {
            c: self.position.c - movement.c,
        }
    }

    pub fn home(&mut self) {
        self.position = self.home;
        self.scaling = 1.0;
    }
}

generate_child_methods!(CanvasContent, camera,
    (camera_get_zoom, get_zoom(), f32),
    (camera_set_pixel_region, set_pixel_region(width: f32, height: f32)),
    (camera_get_base_width, get_base_scale(), ScreenLength),
    (camera_region, region(), Rect),
    (camera_project, project(position: ScreenCoord), Coord),
    (camera_unproject, unproject(position: Coord), ScreenCoord),
    (camera_transform_to_length2d, transform_to_length2d(movement: ScreenLength2d), Length2d),
    (camera_transform_to_length, transform_to_length(length: ScreenLength), Length),
    (camera_zoom_at, zoom_at(movement: f32, coord: ScreenCoord)),
    (camera_pan_by, pan_by(movement: ScreenLength2d)),
    (camera_home, home())
);

impl Camera {
    pub fn get_transform(&self) -> Mat2x3 {
        let mut rtn = Mat2x3::identity()
            .translate(self.region().top_left.c * -1.0)
            .scale(Vec2::new(self.scaling, self.scaling))
            .scale(Vec2::new(self.get_base_scale().c, self.get_base_scale().c))
            .rotate(self.rotation);
        if self.reflect_x {
            rtn.reflect_x();
        }
        if self.reflect_y {
            rtn.reflect_y();
        }

        rtn
    }

    pub fn get_inverse_transform(&self) -> Mat2x3 {
        self.get_transform().inverse()
    }

    pub fn serialize(&self) -> [u8; 22] {
        let mut result = [0u8; 22];

        let camera_scale = self.base_scale.c;

        let scale = camera_scale.to_le_bytes();
        result[0] = scale[0];
        result[1] = scale[1];
        result[2] = scale[2];
        result[3] = scale[3];

        let pos_x = self.position.c.x.to_le_bytes();
        let pos_y = self.position.c.y.to_le_bytes();

        result[4] = pos_x[0];
        result[5] = pos_x[1];
        result[6] = pos_x[2];
        result[7] = pos_x[3];

        result[8] = pos_y[0];
        result[9] = pos_y[1];
        result[10] = pos_y[2];
        result[11] = pos_y[3];

        let scale = self.scaling.to_le_bytes();
        result[12] = scale[0];
        result[13] = scale[1];
        result[14] = scale[2];
        result[15] = scale[3];

        let rotation = self.rotation.to_le_bytes();
        result[16] = rotation[0];
        result[17] = rotation[1];
        result[18] = rotation[2];
        result[19] = rotation[3];

        if self.reflect_x {
            result[20] = 1;
        } else {
            result[20] = 0;
        }

        if self.reflect_y {
            result[21] = 1;
        } else {
            result[21] = 0;
        }

        return result;
    }

    pub fn deserialize(&mut self, data: &[u8]) {
        let mut scale = [0u8; 4];
        scale.copy_from_slice(&data[0..4]);
        let scale = f32::from_le_bytes(scale);
        self.base_scale = ScreenLength::new(scale);

        let mut pos_x = [0u8; 4];
        pos_x.copy_from_slice(&data[4..8]);
        let pos_x = f32::from_le_bytes(pos_x);

        let mut pos_y = [0u8; 4];
        pos_y.copy_from_slice(&data[8..12]);
        let pos_y = f32::from_le_bytes(pos_y);

        self.position = Coord::new(pos_x, pos_y);

        let mut scale = [0u8; 4];
        scale.copy_from_slice(&data[12..16]);
        let scale = f32::from_le_bytes(scale);
        self.scaling = scale;

        let mut rotation = [0u8; 4];
        rotation.copy_from_slice(&data[16..20]);
        let rotation = f32::from_le_bytes(rotation);
        self.rotation = rotation;

        self.reflect_x = data[20] == 1;
        self.reflect_y = data[21] == 1;
    }
}

#[cfg(test)]
mod test {
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn test_transform() {
        let mut camera = Camera::default();

        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        let transform = camera.get_transform();

        assert_approx_eq!(f32, transform.get_translation().x, 250.0);
        assert_approx_eq!(f32, transform.get_translation().y, 250.0);
        assert_approx_eq!(f32, transform.get_scale().x, 500.0);
        assert_approx_eq!(f32, transform.get_scale().y, 500.0);
    }

    #[test]
    fn change_size_transform() {
        let mut camera = Camera::default();

        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 250.0, 250.0);

        let transform = camera.get_transform();

        assert_approx_eq!(f32, transform.get_translation().x, -125.0);
        assert_approx_eq!(f32, transform.get_translation().y, -125.0);
        assert_approx_eq!(f32, transform.get_scale().x, 500.0);
        assert_approx_eq!(f32, transform.get_scale().y, 500.0);
    }

    #[test]
    fn test_zoom_in_center_then_transform() {
        let mut camera = Camera::default();

        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.zoom_at(1.0, ScreenCoord::new(500.0, 500.0));
        camera.zoom_at(1.0, ScreenCoord::new(500.0, 500.0));
        camera.zoom_at(1.0, ScreenCoord::new(500.0, 500.0));
        camera.zoom_at(1.0, ScreenCoord::new(500.0, 500.0));

        let transform = camera.get_transform();

        let minus = ((500.0 * camera.scaling) - 500.0) / 2.0;

        assert_eq!(transform.get_translation().x, 250.0 - minus);
        assert_eq!(transform.get_translation().y, 250.0 - minus);
        assert_eq!(transform.get_scale().x, 500.0 * camera.scaling);
        assert_eq!(transform.get_scale().y, 500.0 * camera.scaling);
    }

    #[test]
    fn zoom_in_corner_top_left_then_transform() {
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 1.5;

        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);
        camera.zoom_at(1.0, ScreenCoord::new(250.0, 250.0));

        let transform = camera.get_transform();

        assert_approx_eq!(f32, camera.scaling, 1.5);
        assert_approx_eq!(f32, transform.get_translation().x, 250.0);
        assert_approx_eq!(f32, transform.get_translation().y, 250.0);
        assert_approx_eq!(f32, transform.get_scale().x, 500.0 * camera.scaling);
        assert_approx_eq!(f32, transform.get_scale().y, 500.0 * camera.scaling);
    }

    #[test]
    fn no_zoom_then_region() {
        let mut camera = Camera::default();
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

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
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.zoom_at(1.0, ScreenCoord::new(250.0, 250.0));
        camera.zoom_at(1.0, ScreenCoord::new(250.0, 250.0));

        let transform = camera.get_transform();

        assert_approx_eq!(f32, camera.scaling, 1.6);
        assert_approx_eq!(f32, transform.get_translation().x, 250.0);
        assert_approx_eq!(f32, transform.get_translation().y, 250.0);
        assert_approx_eq!(f32, transform.get_scale().x, 500.0 * camera.scaling);
        assert_approx_eq!(f32, transform.get_scale().y, 500.0 * camera.scaling);
    }

    #[test]
    fn test_zoom_multiple_in_corner_fast_right_then_transform() {
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 1.25;
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));

        let transform = camera.get_transform();

        let minus = (500.0 * camera.scaling) - 500.0;
        assert_approx_eq!(f32, camera.scaling, 1.6);
        assert_approx_eq!(
            f32,
            transform.get_translation().x,
            250.0 - minus,
            (0.0001, 3)
        );
        assert_approx_eq!(
            f32,
            transform.get_translation().y,
            250.0 - minus,
            (0.0001, 3)
        );
        assert_approx_eq!(
            f32,
            transform.get_scale().x,
            500.0 * camera.scaling,
            (0.0001, 3)
        );
        assert_approx_eq!(
            f32,
            transform.get_scale().y,
            500.0 * camera.scaling,
            (0.0001, 3)
        );
    }

    #[test]
    fn given_camera_when_zoom_in_zoom_out_then_transform_same_then_start() {
        let mut camera = Camera::default();
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(-1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(1.0, ScreenCoord::new(750.0, 750.0));
        camera.zoom_at(-1.0, ScreenCoord::new(750.0, 750.0));

        let transform = camera.get_transform();

        assert_approx_eq!(f32, camera.scaling, 1.00);
        assert_approx_eq!(f32, transform.get_translation().x, 250.0, (0.0001, 3));
        assert_approx_eq!(f32, transform.get_translation().y, 250.0, (0.0001, 3));
        assert_approx_eq!(f32, transform.get_scale().x, 500.0, (0.0001, 3));
        assert_approx_eq!(f32, transform.get_scale().y, 500.0, (0.0001, 3));
    }

    #[test]
    fn test_zoom_multiple_in_corner_right_then_transform() {
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

        let minus = (500.0 * camera.scaling) - 500.0;
        assert_approx_eq!(f32, camera.scaling, 3.4, (0.0001, 3));
        assert_approx_eq!(
            f32,
            transform.get_translation().x,
            250.0 - minus,
            (0.0001, 3)
        );
        assert_approx_eq!(
            f32,
            transform.get_translation().y,
            250.0 - minus,
            (0.0001, 3)
        );
        assert_approx_eq!(
            f32,
            transform.get_scale().x,
            500.0 * camera.scaling,
            (0.0001, 3)
        );
        assert_approx_eq!(
            f32,
            transform.get_scale().y,
            500.0 * camera.scaling,
            (0.0001, 3)
        );
    }

    #[test]
    fn test_zoom_center_then_region() {
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 2.0;
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.zoom_at(1.0, ScreenCoord::new(500.0, 500.0));

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
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);
        camera.zoom_at(1.0, ScreenCoord::new(250.0, 250.0));

        let region = camera.region();

        assert_approx_eq!(f32, camera.scaling, 1.5);
        assert_approx_eq!(f32, region.top_left.c.x, -0.333333333);
        assert_approx_eq!(f32, region.top_left.c.y, -0.333333333);
        assert_approx_eq!(f32, region.bottom_right.c.x, 1.0);
        assert_approx_eq!(f32, region.bottom_right.c.y, 1.0);
    }

    #[test]
    fn test_zoom_top_left_corner_then_project() {
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 1.5;
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);
        camera.zoom_at(1.0, ScreenCoord::new(250.0, 250.0));

        let coord = camera.project(ScreenCoord::new(0.0, 0.0));

        assert_approx_eq!(f32, coord.c.x, -0.333333333);
        assert_approx_eq!(f32, coord.c.y, -0.333333333);
    }
}
