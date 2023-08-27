use iced::{Point, Vector};
use vgc::{
    coord::RefCoordType,
    Vgc,
};

use crate::scene::{canvas_camera::Camera, user_selection::Selected};

use super::super::{point_in_radius, MsgScene};

#[derive(Debug, Clone, Default)]
pub struct MoveCoord {
    previous_movement: Option<Vector>,
}

impl MoveCoord {
    pub fn new() -> Self {
        Self { previous_movement: None }
    }
}

pub fn handle_move(
    event: &MsgScene,
    move_coord: &mut MoveCoord,
    camera: &mut Camera,
    selected: &Selected
) {
    match event {
        MsgScene::ClickMain(_) => {
            move_coord.previous_movement = None;
        }
        MsgScene::DragMain(pressmove) => {
            let movement_diff = camera.project(pressmove.current_coord)
                - camera.project(pressmove.start);

            for shape_selected in &selected.shapes {
                for coord in &shape_selected.coords {
                    let mut coord = coord.borrow_mut();
                    coord.x += movement_diff.x;
                    coord.y += movement_diff.y;

                    if let Some(previous_movement) = move_coord.previous_movement {
                        coord.x -= previous_movement.x;
                        coord.y -= previous_movement.y; 
                    }
                }
            }
            move_coord.previous_movement = Some(movement_diff);
        }
        _ => {}
    }
}

#[allow(clippy::single_match)]
pub fn handle_seprate_handle(event: &MsgScene, camera: &mut Camera, vgc_data: &mut Vgc) {
    match event {
        MsgScene::ClickMain(click) => {
            let mut to_do: Vec<(usize, usize)> = Vec::new();
            vgc_data.visit(&mut |shape_index, coord_type| {
                if let RefCoordType::P1(curve_index, coord) = coord_type {
                    if point_in_radius(
                        &Point::new(coord.x, coord.y),
                        &camera.project(click.start_press),
                        camera.fixed_length(12.0),
                    ) && point_in_radius(
                        &Point::new(coord.x, coord.y),
                        &camera.project(click.end_press),
                        camera.fixed_length(12.0),
                    ) {
                        to_do.push((shape_index, curve_index));
                    }
                }
            });

            for (shape_index, curve_index) in to_do {
                let shape = vgc_data.get_mut_shape(shape_index);
                if let Some(shape) = shape {
                    shape.toggle_separate_join_handle(curve_index);
                }
            }
        }
        _ => {}
    }
}
