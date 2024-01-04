use vgc::Rgba;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{user_selection::Selected, CanvasContent};

#[wasm_bindgen]
pub fn set_color_of(selected: Selected, canvas_content: &mut CanvasContent, color: Rgba) {
    for shape in &selected.shapes {
        if let Some(shape) = canvas_content.vgc_data.get_shape_mut(shape.shape_index) {
            shape.color = color.clone();
        }
    }
}
