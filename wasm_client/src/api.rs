use common::Rgba;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{SceneClient, UserSelectionClient};

#[wasm_bindgen]
impl SceneClient {
    pub fn set_color_of(&mut self, selected: &UserSelectionClient, color: Rgba) {
        self.scene_context.set_color_of(&selected.selection, color);
    }

    pub fn move_coords_of(
        &mut self,
        selected: &UserSelectionClient,
        movement: common::types::ScreenLength2d,
    ) {
        self.scene_context
            .move_coords_of(&selected.selection, movement);
    }

    pub fn add_or_remove_coord(
        &mut self,
        selected: &mut UserSelectionClient,
        mouse_position: common::types::ScreenCoord,
    ) {
        self.scene_context
            .add_or_remove_coord(&mut selected.selection, mouse_position);
    }

    pub fn toggle_handle(&mut self, selected: &UserSelectionClient) {
        self.scene_context.toggle_handle(&selected.selection);
    }

    pub fn save(&self) -> Vec<u8> {
        //self.scene.save()
        unimplemented!()
    }

    pub fn load(_data: Uint8Array) -> Self {
        //self.scene.load(data)
        unimplemented!()
    }
}
