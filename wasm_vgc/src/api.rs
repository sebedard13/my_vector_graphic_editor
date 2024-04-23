use crate::{camera::Camera, user_selection::Selected, CanvasContent};
use common::types::{Coord, ScreenLength2d};
use common::{dbg_str, Rgba};
use common::{math::point_in_radius, types::ScreenCoord};
use js_sys::Uint8Array;
use postcard::{from_bytes, to_allocvec};
use vgc::shape::boolean::{ShapeDifference, ShapeUnion};
use vgc::shape::Shape;
use vgc::Vgc;
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
    let movement = canvas_content
        .camera
        .transform_to_length2d_with_rotation(movement);

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

    for (shape_index, shape) in vgc_data.shapes.iter().enumerate() {
        for (curve_index, curve) in shape.curves.iter().enumerate() {
            let coord = curve.p1.borrow();
            if point_in_radius(
                &coord.c,
                &pos.c,
                &camera
                    .transform_to_length2d(ScreenLength2d::new(12.0, 12.0))
                    .c,
            ) {
                to_do.push((shape_index, curve_index));
            }
        }
    }
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
    let mut min_coord = Coord::new(100.0, 100.0);
    let mut min_t = 0.0;

    for shape_selected in &selected.shapes {
        let shape = vgc_data.get_shape(shape_selected.shape_index).unwrap();

        let (curve_index, t, distance, coord) = shape.closest_curve(&pos);

        if distance < min_distance {
            min_distance = distance;
            min_shape_index = shape_selected.shape_index;
            min_curve_index = curve_index;
            min_coord = coord;
            min_t = t;
        }
    }

    let fixed_length = camera
        .transform_to_length2d(ScreenLength2d::new(10.0, 10.0))
        .c;
    if point_in_radius(&pos.c, &min_coord.c, &fixed_length) {
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
    for (shape_index, shape) in vgc_data.shapes.iter().enumerate() {
        for (curve_index, curve) in shape.curves.iter().enumerate() {
            let coord = curve.p1.borrow();
            if point_in_radius(
                &coord.c,
                &pos.c,
                &camera
                    .transform_to_length2d(ScreenLength2d::new(12.0, 12.0))
                    .c,
            ) {
                to_do.push((shape_index, curve_index));
            }
        }
    }

    for (shape_index, curve_index) in to_do {
        let shape = vgc_data.get_shape_mut(shape_index);
        if let Some(shape) = shape {
            shape.toggle_separate_join_handle(curve_index);
        }
    }
}

#[wasm_bindgen]
pub fn draw_shape(
    selected: &mut Selected,
    canvas_content: &mut CanvasContent,
    mouse_position: ScreenCoord,
) {
    let vgc_data = &mut canvas_content.vgc_data;
    let camera = &mut canvas_content.camera;

    let radius = camera.transform_to_length2d_no_scale(ScreenLength2d::new(50.0, 50.0));

    let pos = camera.project(mouse_position);
    // if click create a new shape on point and ready to new point
    let mut shape = Shape::new_circle(pos, radius.c, Rgba::new(0, 0, 0, 255));
    let mut index: Option<usize> = None;

    log::debug!("{}", dbg_str!("start Union"));
    //for selected shape try to union the new shape
    for shape_selected in &selected.shapes {
        let result = {
            let selected_shape = vgc_data.get_shape(shape_selected.shape_index).unwrap();
            selected_shape.union(&shape)
        };
        log::info!("{}", dbg_str!("Union good"));
        match result {
            ShapeUnion::New(new_shape) => {
                vgc_data.replace_shape(shape_selected.shape_index, new_shape);
                match index {
                    Some(index) => {
                        vgc_data.remove_shape(index);
                    }
                    None => {}
                }

                shape = vgc_data
                    .get_shape(shape_selected.shape_index)
                    .unwrap()
                    .clone();
                index = Some(shape_selected.shape_index);
            }
            ShapeUnion::A => {}
            ShapeUnion::B => {
                vgc_data.replace_shape(shape_selected.shape_index, shape);
                shape = vgc_data
                    .get_shape(shape_selected.shape_index)
                    .unwrap()
                    .clone();
                index = Some(shape_selected.shape_index);
            }
            ShapeUnion::None => {}
        }
    }

    //for unselected shape try to difference the new shape
    for shape_index in 0..vgc_data.shapes.len() {
        if !selected.shapes.iter().any(|s| s.shape_index == shape_index) {
            let selected_shape = vgc_data.get_shape(shape_index).unwrap();
            log::info!(
                "{}",
                dbg_str!(
                    "Difference with A: {} \nB: {}",
                    selected_shape.path(),
                    shape.path()
                )
            );
            let result = selected_shape.difference(&shape);
            match result {
                ShapeDifference::New(mut new_shapes) => {
                    log::debug!("{}", dbg_str!("Difference New shape"));
                    vgc_data.replace_shape(shape_index, new_shapes.swap_remove(0));
                    for new_shape in new_shapes {
                        vgc_data.push_shape(new_shape);
                    }
                }
                ShapeDifference::EraseA => {
                    log::debug!("{}", dbg_str!("Difference erase A"));
                    vgc_data.remove_shape(shape_index);
                }
                ShapeDifference::A => {}
                ShapeDifference::AWithBHole => {
                    log::error!("{}", dbg_str!("AWithBHole"));
                    todo!("Add an hole to A");
                }
            }
        }
    }
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

    let camera_slice = main_slice.get((4 + length)..(4 + length + 26)).unwrap();

    let mut camera = Camera::new(vgc_data.max_rect().center(), f32::NAN, f32::NAN);
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
