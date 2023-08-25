use iced::Point;
use vgc::{
    coord::{CoordType, RefCoordType},
    Vgc,
};

use crate::scene::canvas_camera::Camera;

use super::super::{point_in_radius, MsgScene};

#[derive(Debug, Clone, Default)]
pub struct MoveCoord {
    id_point: Option<CoordType>,
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

            let coords: Vec<(usize, RefCoordType<'_>)> = vgc_data.visit_vec();
            let coord_on_vec: Vec<CoordType> = coords
                .iter()
                .filter_map(|(_, ref_coord)| {
                    let coord = match ref_coord {
                        RefCoordType::Cp0(_, coord) => coord,
                        RefCoordType::Cp1(_, coord) => coord,
                        RefCoordType::P1(_, coord) => coord,
                        RefCoordType::Start(coord) => coord,
                    };

                    let point = &camera.project(mousedown.start_press);
                    if point_in_radius(
                        point,
                        &Point::new(coord.x, coord.y),
                        camera.fixed_length(12.0),
                    ) {
                        Some(ref_coord.to_coord_type())
                    } else {
                        None
                    }
                })
                .collect();

            move_coord.id_point = coord_on_vec.first().cloned();
        }
        MsgScene::DragMain(pressmove) => {
            //If handle, move as pair
            let index_shape = 0;
            if let Some(coord_type) = &move_coord.id_point {
                let point = &camera.project(pressmove.current_coord);
                let shape = vgc_data.get_mut_shape(index_shape).unwrap();
                shape.move_coord(coord_type, point.x, point.y);
            };
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
                vgc_data
                    .get_mut_shape(shape_index)
                    .unwrap()
                    .toggle_separate_join_handle(curve_index);
            }
        }
        _ => {}
    }
}
