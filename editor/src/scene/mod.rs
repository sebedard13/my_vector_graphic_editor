mod canvas_camera;
mod coord_position_tooltip;
mod events;
pub mod functionality;
mod selected_shape;

use iced::widget::canvas;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{Cache, Canvas, Frame, Geometry, Path};
use iced::{keyboard, mouse};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Theme};
use vgc::coord::Coord;
use vgc::generate_from_line;
use vgc::Vgc;

use canvas_camera::Camera;
use selected_shape::SelectedShape;

pub use functionality::Functionality;

pub struct Scene {
    draw_cache: Cache,
    pub camera: Camera,
    pub vgc_data: Vgc,
    pub functionality: Functionality,

    selected_shape: SelectedShape,
}

#[derive(Debug, Clone)]
pub enum MsgScene {
    ChangeBounds(Rectangle),
    ScrollZoom(events::Scroll),
    DragCamera(events::Pressmove),
    HomeCamera(),
    BtnScrollZoom(f32), // 1.0 or -1.0 so Forward or Backward
    Mousemove(events::Mousemove),

    DragMain(events::Pressmove),
    MousedownMain(events::Mousedown),
    ClickMain(events::Click),

    ChangeFunctionality(functionality::Functionality),

    SubmitColor(Color),
}

impl Default for Scene {
    fn default() -> Self {
        let vgc_data = generate_from_line(&[
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 0.0, y: 1.0 },
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 0.0, y: 0.0 },
        ]);

        Self {
            draw_cache: Cache::default(),
            camera: Camera::new(vgc_data.ratio as f32),
            vgc_data: vgc_data,
            functionality: Functionality::default(),
            selected_shape: SelectedShape::default(),
        }
    }
}

pub struct CanvasState {
    scene_size: Size,
    event_merger: events::EventsMerger,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            scene_size: Size::new(0.0, 0.0),
            event_merger: events::EventsMerger::default(),
        }
    }
}

impl Scene {
    pub fn update(&mut self, message: MsgScene) {
        self.draw_cache.clear();

        //camera
        match &message {
            MsgScene::ChangeBounds(bounds) => {
                self.camera.pixel_region = bounds.clone();
            }
            MsgScene::ScrollZoom(scroll) => {
                self.camera.handle_zoom(scroll);
            }
            MsgScene::DragCamera(pressmove) => {
                self.camera.handle_translate(pressmove);
            }
            MsgScene::HomeCamera() => {
                self.camera.home();
            }
            MsgScene::BtnScrollZoom(zoom) => {
                self.camera.handle_btn_zoom(zoom);
            }

            MsgScene::ChangeFunctionality(functionality) => {
                self.functionality = functionality.clone();
                return;
            }
            MsgScene::SubmitColor(color) => {
                let shape = 0;
                self.vgc_data.set_shape_background(shape, color.into_rgba8().into());
            }
            _=>{}
        }

        //What to hover
        match &message {
            MsgScene::Mousemove(mousemove) => {
                selected_shape::change_hover(self, mousemove.current_coord);
            }
            _ => {}
        }

        functionality::match_functionality(self, &message);
    }

    pub fn view(&self) -> Element<MsgScene> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl canvas::Program<MsgScene> for Scene {
    type State = CanvasState;

    fn update(
        &self,
        canvas_state: &mut CanvasState,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<MsgScene>) {
        if canvas_state.scene_size != bounds.size() {
            canvas_state.scene_size = bounds.size();
            return (
                event::Status::Captured,
                Some(MsgScene::ChangeBounds(bounds)),
            );
        }

        let event = canvas_state
            .event_merger
            .push_event(cursor.position(), event);

        if let events::EventStatus::Used(Some(merge_event)) = event {
            use events::MergeEvent as eMe;

            match merge_event {
                eMe::Click(click) if click.button == mouse::Button::Left => {
                    return (event::Status::Captured, Some(MsgScene::ClickMain(click)));
                }
                eMe::Mousedown(mousedown) if mousedown.button == mouse::Button::Left => {
                    return (
                        event::Status::Captured,
                        Some(MsgScene::MousedownMain(mousedown)),
                    );
                }
                eMe::Pressmove(pressmove) if pressmove.button == mouse::Button::Right => {
                    return (
                        event::Status::Captured,
                        Some(MsgScene::DragCamera(pressmove)),
                    );
                }
                eMe::Pressmove(pressmove) if pressmove.button == mouse::Button::Left => {
                    return (event::Status::Captured, Some(MsgScene::DragMain(pressmove)));
                }
                eMe::Pressmove(pressmove) if pressmove.button == mouse::Button::Middle => {
                    return (
                        event::Status::Captured,
                        Some(MsgScene::DragCamera(pressmove)),
                    );
                }
                eMe::Scroll(scroll) => {
                    return (event::Status::Captured, Some(MsgScene::ScrollZoom(scroll)));
                }
                eMe::Mousemove(mousemove) => {
                    return (
                        event::Status::Captured,
                        Some(MsgScene::Mousemove(mousemove)),
                    );
                }
                eMe::KeysDown(key_change) if key_change.new_keys == keyboard::KeyCode::Home => {
                    return (event::Status::Captured, Some(MsgScene::HomeCamera()));
                }
                eMe::KeysDown(key_change) if key_change.new_keys == keyboard::KeyCode::PageUp => {
                    return (event::Status::Captured, Some(MsgScene::BtnScrollZoom(1.0)));
                }
                eMe::KeysDown(key_change) if key_change.new_keys == keyboard::KeyCode::PageDown => {
                    return (event::Status::Captured, Some(MsgScene::BtnScrollZoom(-1.0)));
                }
                eMe::KeysDown(key_change) if key_change.new_keys == keyboard::KeyCode::D && 
                key_change.active_keys.contains(&keyboard::KeyCode::LControl) &&
                key_change.active_keys.contains(&keyboard::KeyCode::LAlt) => {
                    println!("{}",self.vgc_data.debug_string());
                    let cursor_position = cursor.position();
                    if let Some(pos) = cursor_position {
                        println!("cursor: {:?}",self.camera.project(pos));
                    }
                    return (event::Status::Captured, None);
            
                }


                _ => {}
            }
        }

        (event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _: &CanvasState,
        renderer: &Renderer,
        _: &Theme,
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

            let cursor_pos = cursor.position();

            if let Some(pos) = cursor_pos {
                coord_position_tooltip::draw(self, &mut frame, pos);
            }

            frame.with_save(|frame| {
                self.camera.transform_frame(frame, bounds);

                selected_shape::draw(self, frame);
                if let Some(pos) = cursor_pos {
                    selected_shape::draw_closest_pt(self, frame,pos);
                }
               
            });
            frame.into_geometry()
        };

        vec![life, overlay]
    }

    fn mouse_interaction(
        &self,
        _: &CanvasState,
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
