
use iced::alignment;
use iced::keyboard;
use iced::mouse;
use iced::widget::canvas;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{Cache, Canvas, Frame, Geometry, Path, Text};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Theme, Vector};

pub struct Grid {
    draw_cache: Cache,
    translation: Vector,
    scaling: f32,
}

#[derive(Debug, Clone)]
pub enum MsgGrid {
    Translated(Vector),
    Scaled(f32, Option<Vector>),
}

impl Default for Grid {
    fn default() -> Self {
        Self::from_preset()
    }
}

impl Grid {
    const MIN_SCALING: f32 = 0.1;
    const MAX_SCALING: f32 = 2.0;

    pub fn from_preset() -> Self {
        Self {
            draw_cache: Cache::default(),
            translation: Vector::default(),
            scaling: 1.0,
        }
    }


    pub fn update(&mut self, message: MsgGrid) {
        match message {
            MsgGrid::Translated(translation) => {
                self.translation = translation;

                self.draw_cache.clear();
            }
            MsgGrid::Scaled(scaling, translation) => {
                self.scaling = scaling;

                if let Some(translation) = translation {
                    self.translation = translation;
                }

                self.draw_cache.clear();
            }
        }
    }

    pub fn view(&self) -> Element<MsgGrid> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn visible_region(&self, size: Size) -> Region {
        let width = size.width / self.scaling;
        let height = size.height / self.scaling;

        Region {
            x: -self.translation.x - width / 2.0,
            y: -self.translation.y - height / 2.0,
            width,
            height,
        }
    }

    fn project(&self, position: Point, size: Size) -> Point {
        let region = self.visible_region(size);

        Point::new(
            position.x / self.scaling + region.x,
            position.y / self.scaling + region.y,
        )
    }
}

impl canvas::Program<MsgGrid> for Grid {
    type State = Interaction;

    fn update(
        &self,
        interaction: &mut Interaction,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<MsgGrid>) {
        if let Event::Mouse(mouse::Event::ButtonReleased(_)) = event {
            *interaction = Interaction::None;
        }

        let cursor_position = if let Some(position) = cursor.position_in(bounds) {
            position
        } else {
            return (event::Status::Ignored, None);
        };


        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(button) => {
                    let message = match button {
                        mouse::Button::Left => {
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
                        Interaction::Panning { translation, start } => Some(MsgGrid::Translated(
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
                mouse::Event::WheelScrolled { delta } => match delta {
                    mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
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
                },
                _ => (event::Status::Ignored, None),
            },
            Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                let message = match key_code {
                    keyboard::KeyCode::E => Some(MsgGrid::Scaled(
                        self.scaling * 1.1,
                        None,
                    )),
                    keyboard::KeyCode::Q => Some(MsgGrid::Scaled(
                        self.scaling / 1.1,
                        None,
                    )),
                    _ => None,
                };

                (event::Status::Captured, message)
            }

            _ => (event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        _interaction: &Interaction,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);

        let life = self.draw_cache.draw(renderer, bounds.size(), |frame| {
            let background = Path::rectangle(Point::ORIGIN, frame.size());
            frame.fill(&background, Color::from_rgb8(0x40, 0x44, 0x4B));

            frame.with_save(|frame| {
                frame.translate(center);
                frame.scale(self.scaling);
                frame.translate(self.translation);
                frame.scale(20.0);

                frame.fill_rectangle(
                    Point::new(0 as f32, 0 as f32),
                    Size::UNIT,
                    Color::WHITE,
                );
            });
        });

        let overlay = {
            let mut frame = Frame::new(renderer, bounds.size());

            let cursor_pos = cursor
             .position_in(bounds);

            let text = Text {
                color: Color::WHITE,
                size: 14.0,
                position: Point::new(frame.width(), frame.height()),
                horizontal_alignment: alignment::Horizontal::Right,
                vertical_alignment: alignment::Vertical::Bottom,
                ..Text::default()
            };

            if let Some(pos) = cursor_pos {
                let pos = self.project(pos, bounds.size());
                frame.fill_text(Text {
                    content: format!("({:.4}, {:.4}) {:.0}%", pos.x, pos.y,self.scaling*100.0),
                    position: text.position - Vector::new(0.0, 0.0),
                    ..text
                });
            }

            frame.into_geometry()
        };

        vec![life, overlay]
    }

    fn mouse_interaction(
        &self,
        interaction: &Interaction,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        match interaction {
            Interaction::Panning { .. } => mouse::Interaction::Grabbing,
            Interaction::None if cursor.is_over(bounds) => mouse::Interaction::Crosshair,
            _ => mouse::Interaction::default(),
        }
    }
}

pub struct Region {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

pub enum Interaction {
    None,
    Panning { translation: Vector, start: Point },
}

impl Default for Interaction {
    fn default() -> Self {
        Self::None
    }
}
