mod canvas_context_2d_render;

use crate::canvas_context_2d_render::CanvasContext2DRender;
use common::{dbg_str, types::ScreenRect};
use database::SceneUserContext;
use log::{info, warn};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct SceneClient {
    scene: SceneUserContext,
}

#[wasm_bindgen]
impl SceneClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SceneClient {
        let scene = SceneUserContext::new();
        Self { scene }
    }

    pub fn get_render_rect(&self) -> ScreenRect {
        let size = self.scene.camera.get_base_scale();

        let sc = ScreenRect::new(0.0, 0.0, size.c.x, size.c.y);
        sc
    }

    pub fn default_call() -> SceneClient {
        let scene = SceneUserContext::default();
        Self { scene }
    }

    pub fn main_render(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let transform = self.scene.camera.get_transform();
        let mut render = CanvasContext2DRender::new(ctx, transform);

        let pixel_region = self.scene.camera.get_pixel_region();

        ctx.clear_rect(
            pixel_region.top_left.c.x as f64,
            pixel_region.top_left.c.y as f64,
            pixel_region.width() as f64,
            pixel_region.height() as f64,
        );

        self.scene
            .scene_render(&mut render)
            .map_err(|e| JsValue::from_str(&e))
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
