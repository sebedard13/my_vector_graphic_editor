use vgc::Rgba;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{user_selection::Selected, CanvasContent};

#[wasm_bindgen]
pub fn set_color_of(selected: &Selected, canvas_content: &mut CanvasContent, color: Rgba) {
    for shape in &selected.shapes {
        if let Some(shape) = canvas_content.vgc_data.get_shape_mut(shape.shape_index) {
            shape.color = color.clone();
        }
    }
}

#[wasm_bindgen]
pub fn move_coords_of(selected: &Selected, canvas_content: &mut CanvasContent, x: f64, y: f64) {
    let (x, y) = canvas_content.camera.fixed_2d_length((x as f32, y as f32));

    for shape in &selected.shapes {
        for coord in &shape.coords {
            let mut coord = coord.borrow_mut();
            coord.x += x;
            coord.y += y;
        }
    }
}
