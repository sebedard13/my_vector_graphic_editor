mod api;
mod camera_client;
mod canvas_context_2d_render;

use crate::canvas_context_2d_render::CanvasContext2DRender;
use common::{
    dbg_str,
    pures::{Affine, Vec2},
    types::{Coord, ScreenCoord, ScreenRect},
    Rgba,
};
use database::{SceneUserContext, SelectedLevel, UserSelection};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct SceneClient {
    scene_context: SceneUserContext,
}

#[macro_export]
macro_rules! generate_child_methods {
    ($child:ident $(, ($method_par:ident, $method:ident $(($($param:ident : $type:ty),* ))?$(, $rtn:ty)?))+ ) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl SceneClient {
            $(
                pub fn $method_par(&mut self $(, $($param : $type),* )?) $(-> $rtn)? {
                    self.scene_context.$child.$method($( $($param),* )?)
                }
            )*
        }
    };
}

#[wasm_bindgen]
impl SceneClient {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f32, height: f32) -> SceneClient {
        let scene_context = SceneUserContext::new(width, height);
        Self { scene_context }
    }

    pub fn get_render_rect(&self) -> ScreenRect {
        let size = self.scene_context.camera.get_base_scale();

        let sc = ScreenRect::new(0.0, 0.0, size.c.x, size.c.y);
        sc
    }

    pub fn default_call() -> SceneClient {
        let scene_context = SceneUserContext::default();
        Self { scene_context }
    }

    pub fn render_main(
        &self,
        user_selection: &UserSelectionClient,
        ctx: &CanvasRenderingContext2d,
    ) -> Result<(), JsValue> {
        let transform = self.scene_context.camera.get_transform();
        let mut render = CanvasContext2DRender::new(ctx, transform);

        let pixel_region = self.scene_context.camera.get_pixel_region();

        ctx.clear_rect(
            pixel_region.top_left.c.x as f64,
            pixel_region.top_left.c.y as f64,
            pixel_region.width() as f64,
            pixel_region.height() as f64,
        );

        self.scene_context
            .scene_render(&mut render)
            .map_err(|e| JsValue::from_str(&e))?;

        self.scene_context
            .draw(&user_selection.selection, &mut render)
            .map_err(|e| JsValue::from_str(&e))?;
        self.scene_context
            .draw_closest_pt(&user_selection.selection, &mut render)
            .map_err(|e| JsValue::from_str(&e))?;

        Ok(())
    }

    pub fn render_cover(
        &self,
        ctx: &CanvasRenderingContext2d,
        width: f32,
        height: f32,
    ) -> Result<(), JsValue> {
        let max_rect = self.scene_context.scene.max_rect();

        let scale_x = width / max_rect.width();
        let scale_y = height / max_rect.height();

        let mut ctx_2d_renderer = CanvasContext2DRender::new(
            ctx,
            Affine::identity()
                .translate(max_rect.top_left.c * -1.0)
                .scale(Vec2::new(scale_x, scale_y)),
        );

        self.scene_context
            .scene_render(&mut ctx_2d_renderer)
            .map_err(|e| JsValue::from_str(&e))
    }

    pub fn debug_string(&self) -> String {
        self.scene_context.scene.debug_string()
    }
}

#[wasm_bindgen]
pub struct UserSelectionClient {
    #[wasm_bindgen(skip)]
    pub selection: UserSelection,
}

#[macro_export]
macro_rules! generate_selection_child_methods {
    ($(($method_par:ident, $method:ident $(($($param:ident : $type:ty),* ))?$(, $rtn:ty)?))+ ) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl UserSelectionClient {
            $(
                pub fn $method_par(&mut self $(, $($param : $type),* )?) $(-> $rtn)? {
                    self.selection.$method($( $($param),* )?)
                }
            )*
        }
    };
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum SelectedLevelClient {
    None,
    Shape,
    Coord,
}

impl From<SelectedLevel> for SelectedLevelClient {
    fn from(level: SelectedLevel) -> Self {
        match level {
            SelectedLevel::None => SelectedLevelClient::None,
            SelectedLevel::Shape => SelectedLevelClient::Shape,
            SelectedLevel::Coord => SelectedLevelClient::Coord,
        }
    }
}

impl From<SelectedLevelClient> for SelectedLevel {
    fn from(level: SelectedLevelClient) -> Self {
        match level {
            SelectedLevelClient::None => SelectedLevel::None,
            SelectedLevelClient::Shape => SelectedLevel::Shape,
            SelectedLevelClient::Coord => SelectedLevel::Coord,
        }
    }
}

#[wasm_bindgen]
impl UserSelectionClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> UserSelectionClient {
        let selection = UserSelection::new();
        Self { selection }
    }

    pub fn get_selected_level(&self) -> SelectedLevelClient {
        self.selection.get_selected_level().into()
    }

    pub fn clear_to_level(&mut self, level: SelectedLevelClient) {
        self.selection.clear_to_level(level.into())
    }

    pub fn get_selected_colors(&self, canvas_context: &SceneClient) -> Vec<Rgba> {
        self.selection
            .get_selected_colors(&canvas_context.scene_context)
    }

    pub fn change_hover(&mut self, canvas_context: &SceneClient, cursor_position: Coord) {
        self.selection
            .change_hover(&canvas_context.scene_context, cursor_position)
    }

    pub fn change_selection(&mut self, canvas_context: &SceneClient, cursor_position: Coord) {
        self.selection
            .change_selection(&canvas_context.scene_context, cursor_position)
    }

    pub fn add_selection(&mut self, canvas_context: &SceneClient, cursor_position: Coord) {
        self.selection
            .add_selection(&canvas_context.scene_context, cursor_position)
    }

    pub fn set_mouse_position(&mut self, position: Option<Coord>) {
        self.selection.mouse_position = position
    }
}
//------------------------------------------------------------------------------
// Utilities
//------------------------------------------------------------------------------

#[wasm_bindgen]
pub fn set_logger(string: String) {
    console_error_panic_hook::set_once();
    match string.as_str() {
        "trace" => {
            console_log::init_with_level(log::Level::Trace).expect("error initializing log");
            info!("{}", dbg_str!("Trace log level set"));
        }
        "debug" => {
            console_log::init_with_level(log::Level::Debug).expect("error initializing log");
            info!("{}", dbg_str!("Debug log level set"));
        }
        "info" => {
            console_log::init_with_level(log::Level::Info).expect("error initializing log");
            info!("{}", dbg_str!("Info log level set"));
        }
        "warn" => {
            console_log::init_with_level(log::Level::Warn).expect("error initializing log");
        }
        "error" => {
            console_log::init_with_level(log::Level::Error).expect("error initializing log");
        }
        "off" => {}
        _ => {
            console_log::init_with_level(log::Level::Debug).expect("error initializing log");
            warn!("{}", dbg_str!("Invalid log level, defaulting to debug"));
        }
    }
}

//------------------------------------------------------------------------------
// Macros
//------------------------------------------------------------------------------

/// Return a representation of an object owned by JS.
#[macro_export]
macro_rules! value {
    ($value:expr) => {
        wasm_bindgen::JsValue::from($value)
    };
}
