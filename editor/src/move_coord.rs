use iced::{
    event,
    mouse::{self, Cursor},
    widget::canvas::Event,
    Point, Rectangle,
};

use crate::scene::{point_in_radius, Scene, MsgScene};

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

    pub fn update(scene: &mut Scene, msg: MoveCoordStep) {
        match msg {
            MoveCoordStep::Click(_, id) => {
                scene.move_coord.id_point = Some(id);
            }
            MoveCoordStep::Drag(pt) => match scene.move_coord.id_point {
                Some(id) => {
                    scene.vgc_data.move_coord(id, pt.x, pt.y);
                }
                None => {}
            },
            MoveCoordStep::Release => scene.move_coord.id_point = None,
        }
    }

    pub fn handle_event(
        scene: &Scene,
        event: Event,
        cursor_position: Point,
        cursor: Cursor,
        bounds: Rectangle,
    ) -> (iced::event::Status, Option<MsgScene>) {
        match scene.move_coord.id_point {
            Some(_) => match event {
                Event::Mouse(mouse_event) => match mouse_event {
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        return (
                            event::Status::Captured,
                            Some(MsgScene::MoveCoord(MoveCoordStep::Release)),
                        );
                    }
                    mouse::Event::CursorMoved { .. } => {
                        let pt = scene.camera.project(cursor_position, bounds.size());
                        return (
                            event::Status::Captured,
                            Some(MsgScene::MoveCoord(MoveCoordStep::Drag(pt))),
                        );
                    }
                    _ => {}
                },
                _ => {}
            },
            None => match event {
                Event::Mouse(mouse_event) => match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        let coords = scene.vgc_data.list_coord();
                        for coord in coords {
                            match cursor.position_in(bounds) {
                                Some(p) => {
                                    if point_in_radius(
                                        &scene.camera.project(p, bounds.size()),
                                        &Point::new(coord.coord.x, coord.coord.y),
                                        scene.camera.fixed_length(12.0),
                                    ) {
                                        let pt = scene.camera.project(p, bounds.size());
                                        return (
                                            event::Status::Captured,
                                            Some(MsgScene::MoveCoord(MoveCoordStep::Click(
                                                pt, coord.i,
                                            ))),
                                        );
                                    }
                                }
                                None => {}
                            }
                        }

                        return (event::Status::Ignored, None);
                    }
                    _ => {}
                },
                _ => {}
            },
        }

        (event::Status::Ignored, None)
    }
}
