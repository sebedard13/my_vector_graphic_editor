use crate::{camera::Camera, user_selection::Selected, CanvasContent};
use common::types::{ScreenLength, ScreenLength2d};
use common::Rgba;
use common::{math::point_in_radius, types::ScreenCoord};
use js_sys::Uint8Array;
use postcard::{from_bytes, to_allocvec};
use vgc::{coord::RefCoordType, Vgc};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn set_color_of(selected: &Selected, canvas_content: &mut CanvasContent, color: Rgba) {
    for shape in &selected.shapes {
        if let Some(shape) = canvas_content.vgc_data.get_shape_mut(shape.shape_index) {
            shape.color = color.clone();
        }
    }
}

#[wasm_bindgen]
pub fn move_coords_of(
    selected: &Selected,
    canvas_content: &mut CanvasContent,
    movement: ScreenLength2d,
) {
    let movement = canvas_content.camera.transform_to_length2d(movement);

    for shape in &selected.shapes {
        for coord in &shape.coords {
            let mut coord = coord.borrow_mut();
            let res_vec2 = coord.c + movement.c;
            coord.set_x(res_vec2.x);
            coord.set_y(res_vec2.y);
        }
    }
}

#[wasm_bindgen]
pub fn add_or_remove_coord(
    selected: &Selected,
    canvas_content: &mut CanvasContent,
    mouse_position: ScreenCoord,
) {
    let vgc_data = &mut canvas_content.vgc_data;
    let camera = &mut canvas_content.camera;
    let pos = camera.project(mouse_position);

    // if click is on a point, remove it
    let mut to_do: Vec<(usize, usize)> = Vec::new();
    vgc_data.visit(&mut |shape_index, coord_type| {
        if let RefCoordType::P1(curve_index, coord) = coord_type {
            if point_in_radius(
                &coord.c,
                &pos.c,
                camera.transform_to_length(ScreenLength::new(12.0)).c,
            ) {
                to_do.push((shape_index, curve_index));
            }
        }
    });
    if !to_do.is_empty() {
        for (shape_index, curve_index) in to_do {
            let shape = vgc_data.get_shape_mut(shape_index).unwrap();
            shape.remove_curve(curve_index);

            if shape.is_empty() {
                vgc_data.remove_shape(shape_index);
            }
        }

        return;
    }

    // if click is on the path of curve, add a point
    let mut min_distance = std::f32::MAX;
    let mut min_shape_index = 0;
    let mut min_curve_index = 0;
    let mut min_t = 0.0;

    for shape_selected in &selected.shapes {
        let shape = vgc_data.get_shape(shape_selected.shape_index).unwrap();

        let (curve_index, t, distance, _) = shape.closest_curve(&pos);

        if distance < min_distance {
            min_distance = distance;
            min_shape_index = shape_selected.shape_index;
            min_curve_index = curve_index;
            min_t = t;
        }
    }

    let fixed_length = camera.transform_to_length(ScreenLength::new(10.0));
    if min_distance <= fixed_length.c {
        let shape = vgc_data
            .get_shape_mut(min_shape_index)
            .expect("Shape is valid because it was selected");

        shape.insert_coord_smooth(min_curve_index, min_t);
        return;
    }
}

#[wasm_bindgen]
pub fn toggle_handle(
    _: &Selected,
    canvas_content: &mut CanvasContent,
    mouse_position: ScreenCoord,
) {
    let vgc_data = &mut canvas_content.vgc_data;
    let camera = &mut canvas_content.camera;
    let pos = camera.project(mouse_position);

    let mut to_do: Vec<(usize, usize)> = Vec::new();
    vgc_data.visit(&mut |shape_index, coord_type| {
        if let RefCoordType::P1(curve_index, coord) = coord_type {
            if point_in_radius(
                &coord.c,
                &pos.c,
                camera.transform_to_length(ScreenLength::new(12.0)).c,
            ) {
                to_do.push((shape_index, curve_index));
            }
        }
    });

    for (shape_index, curve_index) in to_do {
        let shape = vgc_data.get_shape_mut(shape_index);
        if let Some(shape) = shape {
            shape.toggle_separate_join_handle(curve_index);
        }
    }
}

#[wasm_bindgen]
pub fn draw_shape(_: &Selected, canvas_content: &mut CanvasContent, mouse_position: ScreenCoord) {
    let vgc_data = &mut canvas_content.vgc_data;
    let camera = &mut canvas_content.camera;

    let pos = camera.project(mouse_position);
    // if click create a new shape on point and ready to new point
    vgc::create_circle(vgc_data, pos, 0.1);
}

#[wasm_bindgen]
pub fn load_from_arraybuffer(array: Uint8Array) -> CanvasContent {
    let vec = array.to_vec();
    let main_slice = vec.as_slice();
    let first_4_bytes = main_slice.get(0..4).unwrap();
    let length = u32::from_le_bytes([
        first_4_bytes[0],
        first_4_bytes[1],
        first_4_bytes[2],
        first_4_bytes[3],
    ]) as usize;

    let slice = main_slice.get(4..(4 + length)).unwrap();

    let vgc_data = from_bytes::<Vgc>(slice).expect("Deserialization should be valid");

    let camera_slice = main_slice.get((4 + length)..(4 + length + 22)).unwrap();

    let mut camera = Camera::new(vgc_data.max_rect().center(), f32::NAN);
    camera.deserialize(camera_slice);

    return CanvasContent { vgc_data, camera };
}

#[wasm_bindgen]
pub fn save_to_arraybuffer(canvas_content: &CanvasContent) -> Vec<u8> {
    let vec = to_allocvec::<Vgc>(&canvas_content.vgc_data).expect("Serialization should be valid");

    let length = (vec.len() as u32).to_le_bytes();

    let mut result = Vec::new();

    result.push(length[0]);
    result.push(length[1]);
    result.push(length[2]);
    result.push(length[3]);
    result.extend(vec);

    let camera_slice = canvas_content.camera.serialize();
    result.extend(camera_slice);

    return result;
}
