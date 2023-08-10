use iced::{mouse::{Cursor, self}, Vector, Rectangle, event, Point, keyboard, widget::canvas::Event, Size};
use iced::widget::canvas::Frame;
use crate::grid::{MsgGrid, Interaction};


pub struct Camera {
    pub translation: Vector,
    pub scaling: f32,
    home: Vector,
    ratio : f32,
}

impl Camera{
    pub const MIN_SCALING: f32 = 0.1;
    pub const MAX_SCALING: f32 = 50.0;
    pub const WIDTH: f32 = 500.0;

    pub fn new(ratio :f32) -> Self {
        let default_translate = Vector::new(
            -Self::WIDTH / 2.0,
            -Self::WIDTH / ratio as f32 / 2.0,
        );

        Self {
            translation: default_translate,
            scaling: 1.0,
            home: default_translate,
            ratio : ratio,
        }
    }

    pub fn visible_region(&self, size: Size) -> Region {
        let width = size.width / self.scaling;
        let height = size.height / self.scaling;

        Region {
            x: -self.translation.x - width / 2.0,
            y: -self.translation.y - height / 2.0,
            width,
            height,
        }
    }

    pub  fn project(&self, position: Point, size: Size) -> Point {
        let region = self.visible_region(size);

        Point::new(
            (position.x / self.scaling + region.x) / Self::WIDTH ,
            (position.y / self.scaling + region.y) / (Self::WIDTH/ self.ratio as f32) ,
        )
    }


    fn handle_scroll(&self, y: f32, cursor: Cursor, bounds: Rectangle) -> (iced::event::Status, Option<MsgGrid>) {
        if y < 0.0 && self.scaling > Self::MIN_SCALING
            || y > 0.0 && self.scaling < Self::MAX_SCALING
        {
            let old_scaling = self.scaling;
    
            let scaling = (self.scaling * (1.0 + y / 30.0))
                .clamp(Self::MIN_SCALING, Self::MAX_SCALING);
    
            let translation = if let Some(cursor_to_center) =
                cursor.position_from(bounds.center())
            {
                let factor = scaling - old_scaling;
    
                Some(
                    self.translation
                        - Vector::new(
                            cursor_to_center.x * factor
                                / (old_scaling * old_scaling),
                            cursor_to_center.y * factor
                                / (old_scaling * old_scaling),
                        ),
                )
            } else {
                None
            };
    
            (
                event::Status::Captured,
                Some(MsgGrid::Scaled(scaling, translation)),
            )
        } else {
            (event::Status::Captured, None)
        }
    }
    

    pub fn handle_event_camera(&self, event: Event, interaction: &mut Interaction, cursor_position: Point, cursor: Cursor, bounds: Rectangle) -> (iced::event::Status, Option<MsgGrid>) {
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(button) => {
                    let message = match button {
                        mouse::Button::Right => {
                            *interaction = Interaction::Panning {
                                translation: self.translation,
                                start: cursor_position,
                            };
    
                            None
                        }
                        _ => None,
                    };
    
                    (event::Status::Captured, message)
                }
                mouse::Event::CursorMoved { .. } => {
                    let message = match *interaction {
                        Interaction::Panning { translation, start } => {  
                            Some(MsgGrid::Translated(
                            translation + (cursor_position - start) * (1.0 / self.scaling),
                            )
                        )},
                        _ => None,
                    };
    
                    let event_status = match interaction {
                        Interaction::None => event::Status::Ignored,
                        _ => event::Status::Captured,
                    };
    
                    (event_status, message)
                }
                mouse::Event::WheelScrolled { delta } => match delta {
                    mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                        self.handle_scroll(y, cursor, bounds)
                    }
                },
                _ => (event::Status::Ignored, None),
            },
            Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                let message = match key_code {
                    keyboard::KeyCode::PageUp => Some(MsgGrid::Scaled(
                        (self.scaling * 1.1).clamp(Self::MIN_SCALING, Self::MAX_SCALING),
                        None,
                    )),
                    keyboard::KeyCode::PageDown => Some(MsgGrid::Scaled(
                        (self.scaling / 1.1).clamp(Self::MIN_SCALING, Self::MAX_SCALING),
                        None,
                    )),
                    keyboard::KeyCode::Home => Some(MsgGrid::Scaled(
                        1.0,
                        Some(self.home),
                    )),
                    _ => None,
                };
    
                (event::Status::Captured, message)
            }
    
            _ => (event::Status::Ignored, None),
        }
    }


    pub fn transform_frame(&self,  frame:&mut Frame,  bounds: Rectangle) {
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

pub struct Region {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}


