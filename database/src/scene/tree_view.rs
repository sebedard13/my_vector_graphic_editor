use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::convert::*;
use wasm_bindgen::describe::*;
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

impl WasmDescribeVector for TreeViewModel {
    fn describe_vector() {
        inform(VECTOR);
        TreeViewModel::describe();
    }
}

impl VectorIntoWasmAbi for TreeViewModel {
    type Abi = <
        wasm_bindgen::__rt::std::boxed::Box<[wasm_bindgen::JsValue]>
        as wasm_bindgen::convert::IntoWasmAbi
        >::Abi;

    fn vector_into_abi(vector: wasm_bindgen::__rt::std::boxed::Box<[TreeViewModel]>) -> Self::Abi {
        wasm_bindgen::convert::js_value_vector_into_abi(vector)
    }
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
