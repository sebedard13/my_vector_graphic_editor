#[derive(Debug, Clone)]
pub struct Camera {
    pub position: (f32, f32),
    pub scaling: f32,
    home: (f32, f32),
    ratio: f32,
    pub pixel_region: (f32, f32, f32, f32),
    pub const_zoom: f32,
}

impl Camera {
    pub const MIN_SCALING: f32 = 0.1;
    pub const MAX_SCALING: f32 = 50.0;
    pub const WIDTH: f32 = 500.0;

    pub fn new(ratio: f32) -> Self {
        let default_translate = (0.5, 0.5);

        Self {
            position: default_translate,
            scaling: 1.0,
            home: default_translate,
            ratio,
            pixel_region: (0.0, 0.0, 0.0, 0.0),
            const_zoom: 30.0,
        }
    }

    pub fn region(&self) -> (f32, f32, f32, f32) {
        let width = (self.pixel_region.2 - self.pixel_region.0) / self.scaling / Self::WIDTH;
        let height =
            (self.pixel_region.3 - self.pixel_region.1) / self.scaling / (Self::WIDTH / self.ratio);

        let x = self.position.0 - (width / 2.0);
        let y = self.position.1 - (height / 2.0);

        (x, y, width, height)
    }

    /// Return the canvas coordinates of a given pixel point of the apps window.
    /// (0,0) is the top left corner of the window.
    ///
    pub fn project(&self, position: (f32, f32)) -> (f32, f32) {
        let region = &self.region();

        (
            ((position.0 - self.pixel_region.0) / self.scaling / Self::WIDTH) + region.0,
            ((position.1 - self.pixel_region.1) / self.scaling / (Self::WIDTH / self.ratio))
                + region.1,
        )
    }

    pub fn unproject(&self, position: (f32, f32)) -> (f32, f32) {
        let region = &self.region();

        (
            ((position.0 - region.0) * self.scaling * Self::WIDTH) + self.pixel_region.0,
            ((position.1 - region.1) * self.scaling * (Self::WIDTH / self.ratio))
                + self.pixel_region.1,
        )
    }

    pub fn project_in_view(&self, position: (f32, f32)) -> Option<(f32, f32)> {
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
    }

    pub fn handle_zoom(&mut self, movement: f32, coord: (f32, f32)) {
        if movement < 0.0 && self.scaling > Self::MIN_SCALING
            || movement > 0.0 && self.scaling < Self::MAX_SCALING
        {
            let old_scaling = self.scaling;

            let new_scaling = (self.scaling * (1.0 + movement / self.const_zoom))
                .clamp(Self::MIN_SCALING, Self::MAX_SCALING);

            let projected_coord = self.project(coord);

            let factor = 1.0 - (old_scaling / new_scaling);

            self.position = lerp(self.position, projected_coord, factor);

            self.scaling = new_scaling;
        };
    }

    /*pub fn handle_translate(&mut self, pressmove: &events::Pressmove) {
        let translation = match self.interaction {
            Interaction::Panning { translation, start } => {
                if pressmove.start == start {
                    translation
                } else {
                    self.interaction = Interaction::Panning {
                        translation: self.translation,
                        start: pressmove.start,
                    };
                    self.translation
                }
            }
            _ => {
                self.interaction = Interaction::Panning {
                    translation: self.translation,
                    start: pressmove.start,
                };
                self.translation
            }
        };

        self.translation =
            translation + (pressmove.current_coord - pressmove.start) * (1.0 / self.scaling);
    }*/

    pub fn get_transform(&self) -> (f32, f32, f32, f32) {
        let top_left_on_screen = self.unproject((0.0, 0.0));

        let vgc_width = Self::WIDTH * self.scaling;
        let vgc_height = ((vgc_width as f32) * (1.0 / self.ratio)) as f32;

        return (
            top_left_on_screen.0 as f32,
            top_left_on_screen.1 as f32,
            vgc_width as f32,
            vgc_height as f32,
        );
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

fn contain(rect: (f32, f32, f32, f32), point: (f32, f32)) -> bool {
    rect.0 <= point.0
        && rect.1 <= point.1
        && (rect.0 + rect.2) >= point.0
        && (rect.1 + rect.3) >= point.1
}

fn lerp(a: (f32, f32), b: (f32, f32), t: f32) -> (f32, f32) {
    ((1.0 - t) * a.0 + t * b.0, (1.0 - t) * a.1 + t * b.1)
}

#[cfg(test)]
mod test {
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn test_transform() {
        let mut camera = Camera::new(1.0);

        camera.pixel_region = (0.0, 0.0, 1000.0, 1000.0);

        let transform = camera.get_transform();

        assert_approx_eq!(f32, transform.0, 250.0);
        assert_approx_eq!(f32, transform.1, 250.0);
        assert_approx_eq!(f32, transform.2, 500.0);
        assert_approx_eq!(f32, transform.3, 500.0);
    }

    #[test]
    fn change_size_transform() {
        let mut camera = Camera::new(1.0);

        camera.pixel_region = (0.0, 0.0, 250.0, 250.0);

        let transform = camera.get_transform();

        assert_approx_eq!(f32, transform.0, -125.0);
        assert_approx_eq!(f32, transform.1, -125.0);
        assert_approx_eq!(f32, transform.2, 500.0);
        assert_approx_eq!(f32, transform.3, 500.0);
    }

    #[test]
    fn test_zoom_in_center_then_transform() {
        let mut camera = Camera::new(1.0);
        camera.const_zoom = 2.0;

        camera.pixel_region = (0.0, 0.0, 1000.0, 1000.0);

        camera.handle_zoom(1.0, (500.0, 500.0));
        camera.handle_zoom(1.0, (500.0, 500.0));
        camera.handle_zoom(1.0, (500.0, 500.0));
        camera.handle_zoom(1.0, (500.0, 500.0));

        let transform = camera.get_transform();

        let minus = ((500.0 * camera.scaling) - 500.0) / 2.0;

        assert_eq!(transform.0, 250.0 - minus);
        assert_eq!(transform.1, 250.0 - minus);
        assert_eq!(transform.2, 500.0 * camera.scaling);
        assert_eq!(transform.3, 500.0 * camera.scaling);
    }

    #[test]
    fn zoom_in_corner_top_left_then_transform() {
        let mut camera = Camera::new(1.0);
        camera.const_zoom = 2.0;
        camera.pixel_region = (0.0, 0.0, 1000.0, 1000.0);
        camera.handle_zoom(1.0, (250.0, 250.0));

        let transform = camera.get_transform();

        assert_approx_eq!(f32, camera.scaling, 1.5);
        assert_approx_eq!(f32, transform.0, 250.0);
        assert_approx_eq!(f32, transform.1, 250.0);
        assert_approx_eq!(f32, transform.2, 500.0 * camera.scaling);
        assert_approx_eq!(f32, transform.3, 500.0 * camera.scaling);
    }

    #[test]
    fn test_no_zoom_then_region() {
        let mut camera = Camera::new(1.0);
        camera.pixel_region = (0.0, 0.0, 1000.0, 1000.0);

        let region = camera.region();

        assert_approx_eq!(f32, region.0, -0.5);
        assert_approx_eq!(f32, region.1, -0.5);
        assert_approx_eq!(f32, region.2, 2.0);
        assert_approx_eq!(f32, region.3, 2.0);
    }

    #[test]
    fn test_zoom_multiple_in_corner_then_transform() {
        let mut camera = Camera::new(1.0);
        camera.const_zoom = 4.0;
        camera.pixel_region = (0.0, 0.0, 1000.0, 1000.0);

        camera.handle_zoom(1.0, (250.0, 250.0));
        camera.handle_zoom(1.0, (250.0, 250.0));

        let transform = camera.get_transform();

        assert_approx_eq!(f32, camera.scaling, 1.5625);
        assert_approx_eq!(f32, transform.0, 250.0);
        assert_approx_eq!(f32, transform.1, 250.0);
        assert_approx_eq!(f32, transform.2, 500.0 * camera.scaling);
        assert_approx_eq!(f32, transform.3, 500.0 * camera.scaling);
    }

    #[test]
    fn test_zoom_multiple_in_corner_fast_right_then_transform() {
        let mut camera = Camera::new(1.0);
        camera.pixel_region = (0.0, 0.0, 1000.0, 1000.0);
        camera.const_zoom = 4.0;

        camera.handle_zoom(1.0, (750.0, 750.0));
        camera.handle_zoom(1.0, (750.0, 750.0));

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
        let mut camera = Camera::new(1.0);
        camera.pixel_region = (0.0, 0.0, 1000.0, 1000.0);
        camera.const_zoom = 4.0;

        camera.handle_zoom(1.0, (750.0, 750.0));
        camera.handle_zoom(-1.0, (750.0, 750.0));
        camera.handle_zoom(1.0, (750.0, 750.0));
        camera.handle_zoom(-1.0, (750.0, 750.0));
        camera.handle_zoom(1.0, (750.0, 750.0));

        let transform = camera.get_transform();

        assert_approx_eq!(f32, camera.scaling, 1.0986328);
        assert_approx_eq!(f32, transform.0, 250.0, (0.0001, 3));
        assert_approx_eq!(f32, transform.1, 250.0, (0.0001, 3));
        assert_approx_eq!(f32, transform.2, 500.0, (0.0001, 3));
        assert_approx_eq!(f32, transform.3, 500.0, (0.0001, 3));
    }

    #[test]
    fn test_zoom_multiple_in_corner_right_then_transform() {
        let mut camera = Camera::new(1.0);
        camera.pixel_region = (0.0, 0.0, 1000.0, 1000.0);

        camera.handle_zoom(1.0, (750.0, 750.0));
        camera.handle_zoom(1.0, (750.0, 750.0));
        camera.handle_zoom(1.0, (750.0, 750.0));
        camera.handle_zoom(1.0, (750.0, 750.0));
        camera.handle_zoom(1.0, (750.0, 750.0));
        camera.handle_zoom(1.0, (750.0, 750.0));
        camera.handle_zoom(1.0, (750.0, 750.0));
        camera.handle_zoom(1.0, (750.0, 750.0));
        camera.handle_zoom(1.0, (750.0, 750.0));
        camera.handle_zoom(1.0, (750.0, 750.0));
        camera.handle_zoom(1.0, (750.0, 750.0));
        camera.handle_zoom(1.0, (750.0, 750.0));
        camera.handle_zoom(1.0, (750.0, 750.0));

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
        let mut camera = Camera::new(1.0);
        camera.scaling = 2.0;
        camera.pixel_region = (0.0, 0.0, 1000.0, 1000.0);

        let region = camera.region();

        assert_approx_eq!(f32, region.0, 0.0);
        assert_approx_eq!(f32, region.1, 0.0);
        assert_approx_eq!(f32, region.2, 1.0);
        assert_approx_eq!(f32, region.3, 1.0);
    }

    #[test]
    fn test_zoom_top_left_corner_then_region() {
        let mut camera = Camera::new(1.0);
        camera.const_zoom = 2.0;
        camera.pixel_region = (0.0, 0.0, 1000.0, 1000.0);
        camera.handle_zoom(1.0, (250.0, 250.0));

        let region = camera.region();

        assert_approx_eq!(f32, camera.scaling, 1.5);
        assert_approx_eq!(f32, region.0, -0.333333333);
        assert_approx_eq!(f32, region.1, -0.333333333);
        assert_approx_eq!(f32, region.2, 1.333333333);
        assert_approx_eq!(f32, region.3, 1.333333333);
    }
}
