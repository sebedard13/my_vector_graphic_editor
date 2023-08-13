mod canvas_camera;
mod move_coord;
mod selected_shape;

use iced::alignment;
use iced::mouse;
use iced::widget::canvas;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::Fill;
use iced::widget::canvas::{Cache, Canvas, Frame, Geometry, Path, Text};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Theme, Vector};
use vgc::generate_exemple;
use vgc::Vgc;

use canvas_camera::Camera;
use canvas_camera::Region;
use move_coord::MoveCoord;
use move_coord::MoveCoordStep;
use selected_shape::SelectedShape;
use selected_shape::SelectedShapeEvent;

pub struct Scene {
    draw_cache: Cache,
    pub camera: Camera,
    pub vgc_data: Vgc,
    pub move_coord: MoveCoord,

    selected_shape: SelectedShape,
}

pub trait SceneOverlay {
    type T: std::fmt::Debug + Send;

    fn update(&mut self, msg: Self::T);

    fn handle_event(
        &self,
        scene: &Scene,
        event: Event,
        cursor_position: Option<Point>,
    ) -> (iced::event::Status, Option<Self::T>);

    /// Draw on frame with transform done to have canvas in 0 to 1 coordinate
    fn draw(&self, frame: &mut Frame, scene: &Scene);
}

#[derive(Debug, Clone)]
pub enum MsgScene {
    Translated(Vector),
    Scaled(f32, Option<Vector>),
    MoveCoord(MoveCoordStep),
    HoverCoord(SelectedShapeEvent),
    ChangeBounds(Region),
}

impl Default for Scene {
    fn default() -> Self {
        let vgc_data = generate_exemple();

        Self {
            draw_cache: Cache::default(),
            camera: Camera::new(vgc_data.ratio as f32),
            vgc_data: vgc_data,
            move_coord: MoveCoord::new(),
            selected_shape: SelectedShape::default(),
        }
    }
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

impl Scene {
    pub fn update(&mut self, message: MsgScene) {
        match message {
            MsgScene::Translated(translation) => {
                self.camera.translation = translation;

                self.draw_cache.clear();
            }
            MsgScene::Scaled(scaling, translation) => {
                self.camera.scaling = scaling;

                if let Some(translation) = translation {
                    self.camera.translation = translation;
                }

                self.draw_cache.clear();
            }
            MsgScene::MoveCoord(step) => {
                MoveCoord::update(self, step);

                self.draw_cache.clear();
            }
            MsgScene::HoverCoord(message) => self.selected_shape.update(message),
            MsgScene::ChangeBounds(region) => {
                self.camera.region = region;
            }
        }
    }

    pub fn view(&self) -> Element<MsgScene> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}



impl canvas::Program<MsgScene> for Scene {
    type State = Interaction;

    fn update(
        &self,
        _interaction: &mut Interaction,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<MsgScene>) {
        let visible_region = self.camera.visible_region(bounds.size());
        if self.camera.region != visible_region {
            return (
                event::Status::Captured,
                Some(MsgScene::ChangeBounds(visible_region)),
            );
        }

        let cursor_position = if let Some(position) = cursor.position_in(bounds) {
            position
        } else {
            return (event::Status::Ignored, None);
        };

        let rtn = MoveCoord::handle_event(self, event, cursor_position, cursor, bounds);
        match rtn.0 {
            event::Status::Captured => {
                return rtn;
            }
            _ => {}
        }

        let rtn = self.selected_shape.handle_event(self, event, Some(cursor_position));

        match rtn.0 {
            event::Status::Captured => match rtn.1 {
                Some(msg) => return (rtn.0, Some(MsgScene::HoverCoord(msg))),
                None => return (rtn.0, None),
            },
            _ => {}
        }

        self.camera
            .handle_event_camera(event, _interaction, cursor_position, cursor, bounds)
    }

    fn draw(
        &self,
        _interaction: &Interaction,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let life = self.draw_cache.draw(renderer, bounds.size(), |frame| {
            let background = Path::rectangle(Point::ORIGIN, frame.size());
            frame.fill(&background, Color::from_rgb8(0x40, 0x44, 0x4B));

            frame.with_save(|frame| {
                self.camera.transform_frame(frame, bounds);

                let size = Size {
                    width: 1.0,
                    height: (1.0 / self.vgc_data.ratio as f32),
                };

                let color = Color::from_rgb8(
                    self.vgc_data.background.r,
                    self.vgc_data.background.g,
                    self.vgc_data.background.b,
                );

                frame.fill_rectangle(Point::new(0 as f32, 0 as f32), size, color);

                self.vgc_data.frame_render(frame);
            });
        });

        let overlay = {
            let mut frame = Frame::new(renderer, bounds.size());

            let cursor_pos = cursor.position_in(bounds);

            let text = Text {
                color: Color::WHITE,
                size: 14.0,
                position: Point::new(frame.width(), frame.height()),
                horizontal_alignment: alignment::Horizontal::Right,
                vertical_alignment: alignment::Vertical::Bottom,
                ..Text::default()
            };

            if let Some(pos) = cursor_pos {
                let pos = self.camera.project(pos);

                let content = format!(
                    "({:.4}, {:.4}) {:.0}%",
                    pos.x,
                    pos.y,
                    self.camera.scaling * 100.0
                );

                let overlay_width = content.len() as f32 * 6.58;
                let overlay_height = 16.0;

                frame.fill_rectangle(
                    text.position - Vector::new(overlay_width, overlay_height),
                    Size::new(overlay_width, overlay_height),
                    Fill::from(Color::from_rgba8(0x00, 0x00, 0x00, 0.8)),
                );

                frame.fill_text(Text {
                    content,
                    position: text.position - Vector::new(0.0, 0.0),
                    ..text
                });
            }

            frame.with_save(|frame| {
                self.camera.transform_frame(frame, bounds);

                self.selected_shape.draw(frame, self);
            });
            frame.into_geometry()
        };

        vec![life, overlay]
    }

    fn mouse_interaction(
        &self,
        _: &Interaction,
        _: Rectangle,
        _: mouse::Cursor,
    ) -> mouse::Interaction {
        mouse::Interaction::Crosshair
    }
}

/// Return true if the cursor is in the radius of the center
///```rust
///
/// let cursor = Cursor::Available(Point::new(10.0, 10.0));
/// let center = Point::new(0.0, 0.0);
/// let radius = 5.0;
/// assert_eq!(point_in_radius(cursor, center, radius), false);
/// let cursor = Cursor::Available(Point::new(-3.0, 0.0));
/// assert_eq!(point_in_radius(cursor, center, radius), true);
///```
pub fn point_in_radius(point: &Point, center: &Point, radius: f32) -> bool {
    let x = point.x - center.x;
    let y = point.y - center.y;
    let distance = x * x + y * y;
    distance < (radius * radius)
}
