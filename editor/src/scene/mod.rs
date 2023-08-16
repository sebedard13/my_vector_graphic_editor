mod canvas_camera;
mod move_coord;
mod selected_shape;
mod coord_position_tooltip;
mod events;

use iced::{mouse, keyboard};
use iced::widget::canvas;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{Cache, Canvas, Frame, Geometry, Path};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Theme, Vector};
use vgc::generate_exemple;
use vgc::Vgc;

use canvas_camera::Camera;
use move_coord::MoveCoord;
use move_coord::MoveCoordStep;
use selected_shape::SelectedShape;
use selected_shape::SelectedShapeEvent;

use self::events::MergeEvent;

pub struct Scene {
    draw_cache: Cache,
    pub camera: Camera,
    pub vgc_data: Vgc,
    pub move_coord: MoveCoord,

    selected_shape: SelectedShape,
}

#[derive(Debug, Clone)]
pub enum MsgScene {
    Translated(Vector),
    Scaled(f32, Option<Vector>),
    MoveCoord(MoveCoordStep),
    HoverCoord(SelectedShapeEvent),
    ChangeBounds(Rectangle),
    SetCameraInteraction(canvas_camera::Interaction),
    DragMain(events::Scroll),
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

pub struct CanvasState{
    scene_size: Size,
    event_merger: events::EventsMerger,
}

impl Default for CanvasState{
    fn default() -> Self{
        Self{
            scene_size: Size::new(0.0,0.0),
            event_merger: events::EventsMerger::default(),
        }
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
                move_coord::update(self, step);

                self.draw_cache.clear();
            }
            MsgScene::HoverCoord(message) => selected_shape::update(self,message),
            MsgScene::ChangeBounds(bounds) => {
                self.camera.pixel_region =bounds;
            }
            MsgScene::SetCameraInteraction(interaction) =>self.camera.interaction = interaction,
            MsgScene::DragMain(scroll) => {
                
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


macro_rules! return_if_captured{
    // first arm match add!(1,2), add!(2,3) etc
       ($a:expr, $event:expr)=>{
           {
                let rtn = $a;
                match rtn.0 {
                    event::Status::Captured => {
                        return rtn;
                    }
                    _ => {}
                }
           }
       };
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


       /*  match event{
            Event::Mouse(e) => {
                let event = canvas_state.event_merger.match_mouse_event(cursor.position(),e);
                match event {
                    events::EventStatus::Used(evts) => {
                        match evts{
                            Some(merge_event) => match merge_event{
                                events::MergeEvent::Click(click) if click.button == mouse::Button::Left => {},// Lunch click main event,
                                events::MergeEvent::Pressmove(pressmove) if pressmove.button == mouse::Button::Right => {},// Lunch drag camera
                                events::MergeEvent::Pressmove(pressmove) if pressmove.button == mouse::Button::Left => {},// Lunch drag coord
                                events::MergeEvent::Pressmove(pressmove) if pressmove.button == mouse::Button::Middle => {},// Lunch drag coord
                                events::MergeEvent::Scroll(scroll) =>{
                                    return (event::Status::Captured, Some(MsgScene::DragMain(scroll)));
                                } // Lunch zoom
                                _ => return (event::Status::Captured, None),
                            },
                            None => return (event::Status::Captured, None),
                        }
                    },
                    events::EventStatus::Free => return (event::Status::Ignored, None),
                }

            }
            
            Event::Touch(_) => {},
            Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                /*let message = match key_code {
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
                };*/
            },
            _ =>{}
        }*/

        
        if canvas_state.scene_size != bounds.size(){
            canvas_state.scene_size = bounds.size();
            return (
                event::Status::Captured,
                Some(MsgScene::ChangeBounds(bounds))
            );
        }
        
        let cursor_position = cursor.position_in(bounds);
        return_if_captured!(self.camera.handle_event_camera(event, cursor_position, cursor, bounds),event);
        if let Some(cursor_position) = cursor_position {
            return_if_captured!(move_coord::handle_event(self, event, cursor_position), event);
            return_if_captured!(selected_shape::handle_event(self, event, cursor_position),event);
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

            let cursor_pos = cursor.position_in(bounds);

            
            if let Some(pos) = cursor_pos {

                coord_position_tooltip::draw(self,&mut frame, pos);
            }

            frame.with_save(|frame| {
                self.camera.transform_frame(frame, bounds);

                selected_shape::draw(self,frame);
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
