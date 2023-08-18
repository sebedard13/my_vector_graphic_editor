use iced::Point;
use vgc::{CoordType, Vgc};

use crate::scene::canvas_camera::Camera;

use super::super::{point_in_radius, MsgScene};

#[derive(Debug, Clone)]
pub struct MoveCoord {
    id_point: Option<usize>,
}

impl MoveCoord {
    pub fn new() -> Self {
        Self { id_point: None }
    }
}

pub fn handle_move(
    event: &MsgScene,
    move_coord: &mut MoveCoord,
    camera: &mut Camera,
    vgc_data: &mut Vgc,
) {
    match event {
        MsgScene::ClickMain(_) => {
            move_coord.id_point = None;
        }
        MsgScene::MousedownMain(mousedown) => {
            //If handle, move as pair

            let coords = &vgc_data.list_coord();
            let posi = coords.iter().position(|coord| -> bool {
                let point = &camera.project(mousedown.start_press);
                point_in_radius(
                    point,
                    &Point::new(coord.coord.x, coord.coord.y),
                    camera.fixed_length(12.0),
                )
            });
            move_coord.id_point = posi;
        }
        MsgScene::DragMain(pressmove) => {
            //If handle, move as pair

            let coords = &vgc_data.list_coord();
            if let Some(index) = move_coord.id_point {
                let index = coords[index].i;
                let point = &camera.project(pressmove.current_coord);
                vgc_data.move_coord(index, point.x, point.y);
            };
        }
        _ => {}
    }
}

pub fn handle_seprate_handle(event: &MsgScene, camera: &mut Camera, vgc_data: &mut Vgc) {
    match event {
        MsgScene::ClickMain(click) => {
            let mut to_do: Vec<(usize, usize)> = Vec::new();
            vgc_data.visit(&mut |shape_index, coord_type| match coord_type {
                CoordType::P1(curve_index, coord) => {
                    if point_in_radius(
                        &Point::new(coord.x, coord.y),
                        &camera.project(click.start_press),
                        camera.fixed_length(12.0),
                    ) {
                        to_do.push((shape_index, curve_index));
                    }
                }
                _ => {}
            });

            for (shape_index, curve_index) in to_do {
                println!("separate handle shape {} curve {}", shape_index, curve_index);
                vgc_data.separate_handle(shape_index, curve_index);
            }
        }
        _ => {}
    }
}
