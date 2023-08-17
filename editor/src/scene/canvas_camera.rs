use iced::widget::canvas::Frame;
use iced::{
    Point, Rectangle, Vector,
};

use super::events;

#[derive(Debug, Clone)]
pub enum Interaction {
    None,
    Panning { translation: Vector, start: Point },
}

impl Default for Interaction {
    fn default() -> Self {
        Self::None
    }
}
pub struct Camera {
    pub translation: Vector,
    pub scaling: f32,
    home: Vector,
    ratio: f32,
    pub pixel_region: Rectangle,
    pub interaction: Interaction,
}

impl Camera {
    pub const MIN_SCALING: f32 = 0.1;
    pub const MAX_SCALING: f32 = 50.0;
    pub const WIDTH: f32 = 500.0;

    pub fn new(ratio: f32) -> Self {
        let default_translate = Vector::new(-Self::WIDTH / 2.0, -Self::WIDTH / ratio / 2.0);

        Self {
            translation: default_translate,
            scaling: 1.0,
            home: default_translate,
            ratio: ratio,
            pixel_region: Rectangle::default(),
            interaction: Interaction::default(),
        }
    }

    pub fn region(&self) -> Rectangle {
        let size = self.pixel_region.size();
        let width = size.width / self.scaling / Self::WIDTH;
        let height = size.height / self.scaling / (Self::WIDTH / self.ratio);

        Rectangle {
            x: (-self.translation.x / Self::WIDTH - (width / 2.0)),
            y: (-self.translation.y / (Self::WIDTH / self.ratio) - (height / 2.0)),
            width,
            height,
        }
    }

    /// Return the canvas coordinates of a given pixel point of the apps window.
    /// (0,0) is the top left corner of the window.
    /// 
    pub fn project(&self, position: Point) -> Point {
        let region = &self.region();

        Point::new(
            ((position.x -self.pixel_region.x)/ self.scaling / Self::WIDTH) + region.x,
            ((position.y-self.pixel_region.y) / self.scaling / (Self::WIDTH / self.ratio)) + region.y,
        )
    }

    pub fn project_in_canvas(&self, position: Point) -> Option<Point> {
        let point = self.project(position);

        match self.region().contains(point){
            true => Some(point),
            false => None
        }
    }

    pub fn handle_zoom(&mut self, scroll: &events::Scroll) {
        if scroll.movement.y < 0.0 && self.scaling > Self::MIN_SCALING
            || scroll.movement.y > 0.0 && self.scaling < Self::MAX_SCALING
        {
            let old_scaling = self.scaling;

            let scaling = (self.scaling * (1.0 + scroll.movement.y / 30.0))
                .clamp(Self::MIN_SCALING, Self::MAX_SCALING);

            let cursor_to_center = scroll.coord - self.pixel_region.center();

            let factor = scaling - old_scaling;

            self.translation = self.translation
                - Vector::new(
                    cursor_to_center.x * factor / (old_scaling * old_scaling),
                    cursor_to_center.y * factor / (old_scaling * old_scaling),
                );

            self.scaling = scaling;
        };
    }

    pub fn handle_translate(&mut self, pressmove: &events::Pressmove) {
        let translation = match self.interaction {
            Interaction::Panning { translation, start } => {
                if pressmove.start == start {
                    translation
                }
                else{
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

        self.translation = translation+(pressmove.current_coord - pressmove.start) * (1.0 / self.scaling);
    }

    pub fn transform_frame(&self, frame: &mut Frame, bounds: Rectangle) {
        let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);
        frame.translate(center);
        frame.scale(self.scaling);
        frame.translate(self.translation);
        frame.scale(Self::WIDTH);
    }

    pub fn fixed_length(&self, length_px: f32) -> f32 {
        length_px / self.scaling / Self::WIDTH
    }

   
}
