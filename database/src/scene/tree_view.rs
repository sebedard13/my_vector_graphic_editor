use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use super::{LayerType, Scene};

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TreeViewModel {
    pub layer_id: usize,
    pub layer_type: String,
    pub name: String,
    pub level: usize,
}

impl Scene {
    pub fn get_tree_view(&self) -> Vec<TreeViewModel> {
        let mut result = Vec::new();
        for layer in &self.layers {
            match &layer.value {
                LayerType::Shape(_) => {
                    result.push(TreeViewModel {
                        layer_id: layer.id.value(),
                        name: layer.name.clone(),
                        layer_type: layer.value.type_string(),
                        level: 0,
                    });
                }
                LayerType::Folder => {
                    result.push(TreeViewModel {
                        layer_id: layer.id.value(),
                        name: layer.name.clone(),
                        layer_type: layer.value.type_string(),
                        level: 0,
                    });
                }
            }
        }
        result
    }
}
