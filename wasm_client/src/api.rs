use common::Rgba;
use database::{SceneUserContext, TreeViewModel};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{SceneClient, UserSelectionClient};

#[wasm_bindgen]
impl SceneClient {
    pub fn set_color_of(&mut self, selected: &mut UserSelectionClient, color: Rgba) {
        self.scene_context
            .set_color_of(&mut selected.selection, color);
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

    pub fn draw_shape(&mut self, selected: &mut UserSelectionClient) {
        self.scene_context.draw_shape(&mut selected.selection);
    }

    pub fn save(&self) -> Vec<u8> {
        self.scene_context.save().expect("failed to save")
    }

    pub fn load(data: Uint8Array) -> Self {
        let scene_context = SceneUserContext::load(data.to_vec()).expect("failed to load");
        Self { scene_context }
    }

    pub fn get_tree_view(&self) -> Vec<TreeViewModel> {
        self.scene_context.scene.get_tree_view()
    }

    pub fn move_layer(&mut self, id_to_move: usize, id_position: usize) -> Result<(), String> {
        self.scene_context
            .scene
            .layer_move_at(id_to_move.into(), id_position.into())
    }
}
