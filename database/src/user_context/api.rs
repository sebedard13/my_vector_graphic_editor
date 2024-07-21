use common::types::{Coord, ScreenLength2d};
use common::Rgba;
use common::{math::point_in_radius, types::ScreenCoord};

use crate::LayerId;

use super::user_selection::UserSelection;
use super::SceneUserContext;

impl SceneUserContext {
    pub fn set_color_of(&mut self, selected: &UserSelection, color: Rgba) {
        for shape in &selected.shapes {
            if let Some(shape) = self.scene.shape_select_mut(shape.shape_index) {
                shape.color = color.clone();
            }
        }
    }
    pub fn move_coords_of(&mut self, selected: &UserSelection, movement: ScreenLength2d) {
        let movement = self.camera.transform_to_length2d_with_rotation(movement);

        for selected_shape in &selected.shapes {
            let shape = self
                .scene
                .shape_select_mut(selected_shape.shape_index)
                .expect("not 404");
            for coord_id in &selected_shape.coords {
                let (coord_id, coord) = {
                    let coord = shape.coord_select_mut(*coord_id).expect("not 404");
                    let res_vec2 = coord.coord().c + movement.c;
                    (coord.id, Coord::new(res_vec2.x, res_vec2.y))
                };

                shape.coord_set(coord_id, coord);
            }
        }
    }

    pub fn add_or_remove_coord(
        &mut self,
        selected: &mut UserSelection,
        mouse_position: ScreenCoord,
    ) {
        let vgc_data = &mut self.scene;
        let camera = &mut self.camera;
        let pos = camera.project(mouse_position);

        log::debug!("hover_coord: {:?}", selected.hover_coord);
        if selected.hover_coord.is_some() {
            let hover_coord = selected.hover_coord.take().unwrap();
            let shape = vgc_data
                .shape_select_mut(hover_coord.shape_index)
                .expect("Not 404");
            shape.coord_delete(hover_coord.id).expect("Not 404");

            if shape.is_empty() {
                vgc_data.layer_delete(hover_coord.shape_index);
                selected.remove_shape(hover_coord.shape_index);
            }
            return;
        }

        // if click is on the path of curve, add a point
        let mut min_distance = std::f32::MAX;
        let mut min_shape_index = LayerId::null();
        let mut min_curve_index = 0;
        let mut min_coord = Coord::new(100.0, 100.0);
        let mut min_t = 0.0;

        for shape_selected in &selected.shapes {
            let shape = vgc_data.shape_select(shape_selected.shape_index).unwrap();

            let (curve_index, t, distance, coord) = shape.closest_curve(&pos);

            if distance < min_distance {
                min_distance = distance;
                min_shape_index = shape_selected.shape_index;
                min_curve_index = curve_index;
                min_coord = coord;
                min_t = t;
            }
        }

        let fixed_length = camera.transform_to_length2d(ScreenLength2d::new(10.0, 10.0));
        if point_in_radius(&pos.c, &min_coord.c, &fixed_length.c) {
            let shape = vgc_data
                .shape_select_mut(min_shape_index)
                .expect("Shape is valid because it was selected");

            shape.curve_insert_smooth(min_curve_index, min_t);
            return;
        }
    }

    pub fn toggle_handle(&mut self, selected: &UserSelection) {
        let vgc_data = &mut self.scene;

        if let Some(hover_coord) = &selected.hover_coord {
            let shape = vgc_data
                .shape_select_mut(hover_coord.shape_index)
                .expect("Not 404");
            log::debug!("hover_coord: {:?}, {:?}", shape, hover_coord);
            let curve_index = shape
                .curve_select_of_coord_id(hover_coord.id)
                .expect("Not 404");
            shape.toggle_separate_join_handle(curve_index);
        }
    }
}

// pub fn load_from_arraybuffer(array: Uint8Array) -> CanvasContent {
//     let vec = array.to_vec();
//     let main_slice = vec.as_slice();
//     let first_4_bytes = main_slice.get(0..4).unwrap();
//     let length = u32::from_le_bytes([
//         first_4_bytes[0],
//         first_4_bytes[1],
//         first_4_bytes[2],
//         first_4_bytes[3],
//     ]) as usize;

//     let slice = main_slice.get(4..(4 + length)).unwrap();

//     let vgc_data = from_bytes::<Vgc>(slice).expect("Deserialization should be valid");

//     let camera_slice = main_slice.get((4 + length)..(4 + length + 26)).unwrap();

//     let mut camera = Camera::new(vgc_data.max_rect().center(), f32::NAN, f32::NAN);
//     camera.deserialize(camera_slice);

//     return CanvasContent { vgc_data, camera };
// }

// pub fn save_to_arraybuffer(canvas_content: &CanvasContent) -> Vec<u8> {
//     let vec = to_allocvec::<Vgc>(&canvas_content.vgc_data).expect("Serialization should be valid");

//     let length = (vec.len() as u32).to_le_bytes();

//     let mut result = Vec::new();

//     result.push(length[0]);
//     result.push(length[1]);
//     result.push(length[2]);
//     result.push(length[3]);
//     result.extend(vec);

//     let camera_slice = canvas_content.camera.serialize();
//     result.extend(camera_slice);

//     return result;
// }
