use iced::Point;
use vgc::{coord::{Coord, RefCoordType}, Vgc};

use crate::scene::{canvas_camera::Camera, user_selection::Selected, MsgScene, point_in_radius};

#[allow(clippy::single_match)]
pub fn handle_create_or_add_point(
    event: &MsgScene,
    camera: &mut Camera,
    vgc_data: &mut Vgc,
    selected: &Selected,
) {
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
            if !to_do.is_empty(){
                for (shape_index, curve_index) in to_do {
                    let shape = vgc_data.get_shape_mut(shape_index).unwrap();
                    shape.remove_curve(curve_index);
    
                }
                return;
            }

            let mut min_distance = std::f32::MAX;
            let mut min_shape_index = 0;
            let mut min_curve_index = 0;
            let mut min_t = 0.0;
            let pos = camera.project(click.end_press);
            for shape_selected in &selected.shapes {
                let shape = vgc_data.get_shape(shape_selected.shape_index).unwrap();

                let (curve_index, t, distance, _) = shape.closest_curve(&Coord::new(pos.x, pos.y));

                if distance < min_distance {
                    min_distance = distance;
                    min_shape_index = shape_selected.shape_index;
                    min_curve_index = curve_index;
                    min_t = t;
                }
            }

            if min_distance > camera.fixed_length(10.0) {
                return;
            }

            let shape = vgc_data
                .get_shape_mut(min_shape_index)
                .expect("Shape is valid because it was selected");

            shape.insert_coord_smooth(min_curve_index, min_t);
        }
        _ => {}
    }
}
