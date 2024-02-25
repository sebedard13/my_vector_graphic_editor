use common::math::lerp;
use common::pures::{Mat2x3, Vec2};
use common::types::{Coord, Length2d, Rect, ScreenCoord, ScreenLength2d, ScreenRect};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{console_log, CanvasContent};

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
    base_scale: ScreenLength2d,

    home: Coord,
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
    pub fn new(default_translate: Coord, width: f32, height: f32) -> Self {
        console_log!("default_translate: {:?}", default_translate);
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
        let res = movement.c / self.scaling;
        let res = Vec2::new(
            res.x / self.get_base_scale().c.x / 0.5,
            res.y / self.get_base_scale().c.y / 0.5,
        );

        Length2d { c: res }
    }

    pub fn transform_to_length2d_no_scale(&self, length: ScreenLength2d) -> Length2d {
        let res = Vec2::new(
            length.c.x / self.get_base_scale().c.x / 0.5,
            length.c.y / self.get_base_scale().c.y / 0.5,
        );

        Length2d { c: res }
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
        self.rotation = 0.0;
        self.reflect_x = false;
        self.reflect_y = false;
    }
}

generate_child_methods!(CanvasContent, camera,
    (camera_get_zoom, get_zoom(), f32),
    (camera_set_pixel_region, set_pixel_region(width: f32, height: f32)),
    (camera_set_rotation, set_rotation(rotation: f32)),
    (camera_get_rotation, get_rotation(), f32),
    (camera_set_reflect_x, set_reflect_x(reflect_x: bool)),
    (camera_get_reflect_x, get_reflect_x(), bool),
    (camera_set_reflect_y, set_reflect_y(reflect_y: bool)),
    (camera_get_reflect_y, get_reflect_y(), bool),
    (camera_region, region(), Rect),
    (camera_project, project(position: ScreenCoord), Coord),
    (camera_unproject, unproject(position: Coord), ScreenCoord),
    (camera_transform_to_length2d, transform_to_length2d(movement: ScreenLength2d), Length2d),
    (camera_zoom_at, zoom_at(movement: f32, coord: ScreenCoord)),
    (camera_pan_by, pan_by(movement: ScreenLength2d)),
    (camera_home, home())
);

impl Camera {
    pub fn get_transform(&self) -> Mat2x3 {
        let translate_center = self.settings.pixel_region.center().c;

        let m_rot = Mat2x3::identity()
            .translate(translate_center * -1.0)
            .rotate(-self.rotation)
            .translate(translate_center);

        let m_scale = Mat2x3::identity()
            .scale(Vec2::new(0.5, 0.5))
            .scale(Vec2::new(self.scaling, self.scaling))
            .scale(self.get_base_scale().c);

        let m_translate = Mat2x3::from_translate(self.region().top_left.c * -1.0);

        let mut rtn = m_rot * m_scale * m_translate;

        if self.reflect_x {
            let reflect_x = Mat2x3::identity()
                .translate(translate_center * -1.0)
                .reflect_x()
                .translate(translate_center);
            rtn = reflect_x * rtn;
        }
        if self.reflect_y {
            let reflect_y = Mat2x3::identity()
                .translate(translate_center * -1.0)
                .reflect_y()
                .translate(translate_center);
            rtn = reflect_y * rtn;
        }

        rtn
    }

    pub fn get_inverse_transform(&self) -> Mat2x3 {
        self.get_transform().inverse()
    }

    pub fn serialize(&self) -> [u8; 26] {
        let mut result = [0u8; 26];

        let base_width = self.base_scale.c.x.to_le_bytes();
        let base_height = self.base_scale.c.y.to_le_bytes();
        result[0] = base_width[0];
        result[1] = base_width[1];
        result[2] = base_width[2];
        result[3] = base_width[3];

        result[4] = base_height[0];
        result[5] = base_height[1];
        result[6] = base_height[2];
        result[7] = base_height[3];

        let pos_x = self.position.c.x.to_le_bytes();
        let pos_y = self.position.c.y.to_le_bytes();

        result[8] = pos_x[0];
        result[9] = pos_x[1];
        result[10] = pos_x[2];
        result[11] = pos_x[3];

        result[12] = pos_y[0];
        result[13] = pos_y[1];
        result[14] = pos_y[2];
        result[15] = pos_y[3];

        let scale = self.scaling.to_le_bytes();
        result[16] = scale[0];
        result[17] = scale[1];
        result[18] = scale[2];
        result[19] = scale[3];

        let rotation = self.rotation.to_le_bytes();
        result[20] = rotation[0];
        result[21] = rotation[1];
        result[22] = rotation[2];
        result[23] = rotation[3];

        if self.reflect_x {
            result[24] = 1;
        } else {
            result[24] = 0;
        }

        if self.reflect_y {
            result[25] = 1;
        } else {
            result[25] = 0;
        }

        return result;
    }

    pub fn deserialize(&mut self, data: &[u8]) {
        let mut scale_width = [0u8; 4];
        scale_width.copy_from_slice(&data[0..4]);
        let scale_width = f32::from_le_bytes(scale_width);

        let mut scale_height = [0u8; 4];
        scale_height.copy_from_slice(&data[4..8]);
        let scale_height = f32::from_le_bytes(scale_height);

        self.base_scale = ScreenLength2d::new(scale_width, scale_height);

        let mut pos_x = [0u8; 4];
        pos_x.copy_from_slice(&data[8..12]);
        let pos_x = f32::from_le_bytes(pos_x);

        let mut pos_y = [0u8; 4];
        pos_y.copy_from_slice(&data[12..16]);
        let pos_y = f32::from_le_bytes(pos_y);

        self.position = Coord::new(pos_x, pos_y);

        let mut scale = [0u8; 4];
        scale.copy_from_slice(&data[16..20]);
        self.scaling = f32::from_le_bytes(scale);

        let mut rotation = [0u8; 4];
        rotation.copy_from_slice(&data[20..24]);
        self.rotation = f32::from_le_bytes(rotation);

        if data[24] == 1 {
            self.reflect_x = true;
        } else {
            self.reflect_x = false;
        }

        if data[25] == 1 {
            self.reflect_y = true;
        } else {
            self.reflect_y = false;
        }
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

        assert_approx_eq!(f32, region.top_left.c.x, -2.0);
        assert_approx_eq!(f32, region.top_left.c.y, -2.0);
        assert_approx_eq!(f32, region.bottom_right.c.x, 2.0);
        assert_approx_eq!(f32, region.bottom_right.c.y, 2.0);
    }

    #[test]
    fn when_zoom_center_then_region() {
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 2.0;
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        camera.zoom_at(1.0, ScreenCoord::new(500.0, 500.0));

        let region = camera.region();

        assert_approx_eq!(f32, region.top_left.c.x, -1.0);
        assert_approx_eq!(f32, region.top_left.c.y, -1.0);
        assert_approx_eq!(f32, region.bottom_right.c.x, 1.0);
        assert_approx_eq!(f32, region.bottom_right.c.y, 1.0);
    }

    #[test]
    fn when_zoom_top_left_corner_then_region() {
        let mut camera = Camera::default();
        camera.settings.zoom_slope = 1.5;
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);
        camera.zoom_at(1.0, ScreenCoord::new(250.0, 250.0));

        let region = camera.region();

        assert_approx_eq!(f32, camera.scaling, 1.5);
        assert_approx_eq!(f32, region.top_left.c.x, -1.66666666);
        assert_approx_eq!(f32, region.top_left.c.y, -1.66666666);
        assert_approx_eq!(f32, region.bottom_right.c.x, 1.0);
        assert_approx_eq!(f32, region.bottom_right.c.y, 1.0);
    }

    #[test]
    fn given_default_then_transform() {
        let mut camera = Camera::default();

        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        let transform = camera.get_transform();

        assert_approx_eq!(
            Vec2,
            camera.unproject(Coord::new(-1.0, -1.0)).c,
            Vec2::new(250.0, 250.0)
        );
        assert_approx_eq!(
            Vec2,
            camera.unproject(Coord::new(1.0, 1.0)).c,
            Vec2::new(750.0, 750.0)
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

        assert_approx_eq!(f32, coord.c.x, -1.666666666);
        assert_approx_eq!(f32, coord.c.y, -1.666666666);
    }

    #[test]
    fn when_rotation_90deg_then_unproject() {
        let mut camera = Camera::default();
        camera.settings.pixel_region = ScreenRect::new(0.0, 0.0, 1000.0, 1000.0);

        let coord = camera.unproject(Coord::new(-1.0, -1.0));
        assert_approx_eq!(Vec2, coord.c, Vec2::new(250.0, 250.0));

        let coord = camera.unproject(Coord::new(1.0, -1.0));
        assert_approx_eq!(Vec2, coord.c, Vec2::new(750.0, 250.0));

        camera.set_rotation(f32::to_radians(90.0));

        let coord = camera.unproject(Coord::new(-1.0, -1.0));
        assert_approx_eq!(Vec2, coord.c, Vec2::new(250.0, 750.0));

        let coord = camera.unproject(Coord::new(1.0, -1.0));
        assert_approx_eq!(Vec2, coord.c, Vec2::new(250.0, 250.0));
    }
}
