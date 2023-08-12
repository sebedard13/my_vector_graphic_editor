use iced::alignment;
use iced::mouse;
use iced::mouse::Cursor;
use iced::widget::canvas;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::Fill;
use iced::widget::canvas::{Cache, Canvas, Frame, Geometry, Path, Text};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Theme, Vector};
use vgc::generate_exemple;
use vgc::Vgc;

use crate::canvas_camera::Camera;
use crate::move_coord::MoveCoord;
use crate::move_coord::MoveCoordStep;

pub struct Scene {
    draw_cache: Cache,
    pub camera: Camera,
    pub vgc_data: Vgc,
    pub move_coord: MoveCoord,

    selected_shape: SelectedShape,
}

trait SceneOverlay {
    type T :std::fmt::Debug + Send;

    fn update(&mut self, msg : Self::T);

    fn handle_event(
        &self,
        scene : &Scene,
        event: Event,
        cursor_position: Option<Point>,
    ) -> (iced::event::Status, Option<MsgScene>);

    /// Draw on frame with transform done to have canvas in 0 to 1 coordinate
    fn draw(&self, frame: &mut Frame, scene: &Scene);
}

#[derive(Debug, Clone)]
pub enum MsgScene {
    Translated(Vector),
    Scaled(f32, Option<Vector>),
    MoveCoord(MoveCoordStep),
    HoverCoord(usize)
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
            MsgScene::HoverCoord(_) => {
                self.selected_shape.update(message)
            },
            
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
        interaction: &mut Interaction,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<MsgScene>) {
       

        if let Event::Mouse(mouse::Event::ButtonReleased(_)) = event {
            *interaction = Interaction::None;
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

        let rtn = self.selected_shape.handle_event(self,  event, Some(cursor_position));

        match rtn.0 {
            event::Status::Captured => {
                return rtn;
            }
            _ => {}
        }

        self.camera
            .handle_event_camera(event, interaction, cursor_position, cursor, bounds)
    }

    fn draw(
        &self,
        _interaction: &Interaction,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        self.camera.visible_region(bounds.size());
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

/// Return true if the cursor is in the radius of the center
///```rust
///
/// let cursor = Cursor::Available(Point::new(10.0, 10.0));
/// let center = Point::new(0.0, 0.0);
/// let radius = 5.0;
/// assert_eq!(position_in_radius(cursor, center, radius), false);
/// let cursor = Cursor::Available(Point::new(-3.0, 0.0));
/// assert_eq!(position_in_radius(cursor, center, radius), true);
///```
pub fn position_in_radius(cursor: &Cursor, center: &Point, radius: f32) -> bool {
    cursor
        .position()
        .filter(|p| point_in_radius(p, center, radius))
        .is_some()
}

pub fn point_in_radius(point: &Point, center: &Point, radius: f32) -> bool {
    let x = point.x - center.x;
    let y = point.y - center.y;
    let distance = x * x + y * y;
    distance < (radius * radius)
}

struct SelectedShape {
    index_selected_coord: usize,
}

impl Default for SelectedShape{
    fn default() -> Self {
        Self { index_selected_coord: 999 }
    }
} 

impl SceneOverlay for SelectedShape {
    fn draw(&self, frame: &mut Frame, scene: &Scene) {
        // Render points
        let coords = scene.vgc_data.list_coord();
        for coord in coords {
            let color = match self.index_selected_coord == coord.i {
                true => Color::from_rgb8(0x0E, 0x90, 0xAA),
                false => Color::from_rgb8(0x3A, 0xD1, 0xEF),
            };

            let center = Point::new(
                coord.coord.x,
                coord.coord.y * 1.0 / scene.vgc_data.ratio as f32,
            );
            frame.fill(
                &Path::circle(center, scene.camera.fixed_length(5.0)),
                Fill::from(color),
            );
        }
    }

    fn handle_event(
        &self,
        scene: &Scene,
        event: Event,
        cursor_position: Option<Point>,
    ) -> (iced::event::Status, Option<MsgScene>) {
        
        let coords = scene.vgc_data.list_coord();
        for coord in coords {
            match cursor_position {
                Some(p) => {
                    if point_in_radius(
                        &scene.camera.project(p),
                        &Point::new(coord.coord.x, coord.coord.y),
                        scene.camera.fixed_length(12.0),
                    ) {
                      
                        return (iced::event::Status::Captured, Some(MsgScene::HoverCoord(coord.i)));
                    } else {
                       
                    }
                }
                None => {}
            }
        }

        return (iced::event::Status::Ignored, None);
    }

    type T = MsgScene ;

    fn update(&mut self, msg : Self::T) {
       match msg{
        MsgScene::Translated(_) => {},
        MsgScene::Scaled(_, _) => {},
        MsgScene::MoveCoord(_) => {},
        MsgScene::HoverCoord(index) => self.index_selected_coord=index,
    }
    }
}
