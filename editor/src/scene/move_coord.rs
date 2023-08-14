use iced::{
    event,
    mouse::{self},
    widget::canvas::Event,
    Point,
};

use super::{point_in_radius, MsgScene, Scene};

pub struct MoveCoord {
    id_point: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum MoveCoordStep {
    Click(Point, usize),
    Drag(Point),
    Release,
}

impl MoveCoord {
    pub fn new() -> Self {
        Self { id_point: None }
    }
}

pub fn update(scene: &mut Scene, msg: MoveCoordStep) {
    match msg {
        MoveCoordStep::Click(_, id) => {
            scene.move_coord.id_point = Some(id);
        }
        MoveCoordStep::Drag(pt) => if let Some(id) = scene.move_coord.id_point {
           scene.vgc_data.move_coord(id, pt.x, pt.y);
        },
        MoveCoordStep::Release => scene.move_coord.id_point = None,
    }
}

pub fn handle_event(
    scene: &Scene,
    event: Event,
    cursor_position: Point,
) -> (iced::event::Status, Option<MsgScene>) {
    match scene.move_coord.id_point {
        Some(_) => if let Event::Mouse(mouse_event) = event {
           match mouse_event {
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    return (
                        event::Status::Captured,
                        Some(MsgScene::MoveCoord(MoveCoordStep::Release)),
                    );
                }
                mouse::Event::CursorMoved { .. } => {
                    let pt = scene.camera.project(cursor_position);
                    return (
                        event::Status::Captured,
                        Some(MsgScene::MoveCoord(MoveCoordStep::Drag(pt))),
                    );
                }
                _ => {}
            }
        },
        None => if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
                let coords = scene.vgc_data.list_coord();
                for coord in coords {
                    if point_in_radius(
                        &scene.camera.project(cursor_position),
                        &Point::new(coord.coord.x, coord.coord.y),
                        scene.camera.fixed_length(12.0),
                    ) {
                        let pt = scene.camera.project(cursor_position);
                        return (
                            event::Status::Captured,
                            Some(MsgScene::MoveCoord(MoveCoordStep::Click(pt, coord.i))),
                        );
                    }
                }

                return (event::Status::Ignored, None);
            }
        }

    (event::Status::Ignored, None)
}
