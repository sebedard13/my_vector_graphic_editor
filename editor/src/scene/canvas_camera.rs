use crate::scene::MsgScene;
use iced::widget::canvas::Frame;
use iced::{
    event, keyboard,
    mouse::{self, Cursor},
    widget::canvas::Event,
    Point, Rectangle, Size, Vector,
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

    pub fn project(&self, position: Point) -> Point {
        let region = &self.region();

        Point::new(
            (position.x / self.scaling / Self::WIDTH) + region.x,
            (position.y / self.scaling / (Self::WIDTH / self.ratio)) + region.y,
        )
    }

    pub fn handle_scroll(&mut self, scroll: events::Scroll) {
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

    pub fn handle_event_camera(
        &self,
        event: Event,
        cursor_position: Option<Point>,
        _: Cursor,
        _: Rectangle,
    ) -> (iced::event::Status, Option<MsgScene>) {
        let interaction = &self.interaction;

        if let Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Right)) = event {
            return (
                event::Status::Captured,
                Some(MsgScene::SetCameraInteraction(Interaction::None)),
            );
        }

        let cursor_position = if let Some(cursor_position) = cursor_position {
            cursor_position
        } else {
            return (event::Status::Ignored, None);
        };

        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(mouse::Button::Right) => {
                    let interaction = Interaction::Panning {
                        translation: self.translation,
                        start: cursor_position,
                    };
                    (
                        event::Status::Captured,
                        Some(MsgScene::SetCameraInteraction(interaction)),
                    )
                }

                mouse::Event::CursorMoved { .. } => {
                    let message = match *interaction {
                        Interaction::Panning { translation, start } => Some(MsgScene::Translated(
                            translation + (cursor_position - start) * (1.0 / self.scaling),
                        )),
                        _ => None,
                    };

                    let event_status = match interaction {
                        Interaction::None => event::Status::Ignored,
                        _ => event::Status::Captured,
                    };

                    (event_status, message)
                }
                _ => (event::Status::Ignored, None),
            },
            Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                let message = match key_code {
                    keyboard::KeyCode::PageUp => Some(MsgScene::Scaled(
                        (self.scaling * 1.1).clamp(Self::MIN_SCALING, Self::MAX_SCALING),
                        None,
                    )),
                    keyboard::KeyCode::PageDown => Some(MsgScene::Scaled(
                        (self.scaling / 1.1).clamp(Self::MIN_SCALING, Self::MAX_SCALING),
                        None,
                    )),
                    keyboard::KeyCode::Home => Some(MsgScene::Scaled(1.0, Some(self.home))),
                    _ => None,
                };

                (event::Status::Captured, message)
            }

            _ => (event::Status::Ignored, None),
        }
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
