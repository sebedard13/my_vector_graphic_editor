pub mod api;
mod camera;
mod canvas_context_2d_render;
pub mod user_selection;

use crate::canvas_context_2d_render::CanvasContext2DRender;
use camera::Camera;
use common::types::{Coord, ScreenRect};
use vgc::Vgc;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::CanvasRenderingContext2d;

pub use common;

// Function to read from index 1 of our buffer
// And return the value at the index
#[wasm_bindgen]
pub struct CanvasContent {
    #[wasm_bindgen(skip)]
    pub vgc_data: Vgc,

    #[wasm_bindgen(skip)]
    pub camera: Camera,
}

#[wasm_bindgen]
impl CanvasContent {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f32, height: f32) -> CanvasContent {
        console_log!("new width: {}, height: {}", width, height);
        let (max_w, max_h, max_size) = {
            if width > height {
                (width / height, 1.0, -width / height)
            } else {
                (1.0, height / width, height / width)
            }
        };

        let mut vgc_data = vgc::generate_from_line(vec![vec![
            Coord::new(0.0 * max_w, 0.0 * max_h),
            Coord::new(0.0 * max_w, 1.0 * max_h),
            Coord::new(1.0 * max_w, 1.0 * max_h),
            Coord::new(1.0 * max_w, 0.0 * max_h),
        ]]);
        vgc_data.max_size = max_size;

        let shape = vgc_data.get_shape_mut(0).expect("Valid");
        shape.color.r = 255;
        shape.color.g = 255;
        shape.color.b = 255;

        let scale = f32::min(width, height);

        let camera = Camera::new(vgc_data.max_rect().center(), scale);

        Self { camera, vgc_data }
    }

    pub fn get_render_rect(&self) -> ScreenRect {
        let width = self.camera.get_base_scale();

        let rect = self.vgc_data.max_rect();

        let sc = ScreenRect::new(0.0, 0.0, width.c * rect.width(), width.c * rect.height());
        sc
    }

    #[wasm_bindgen]
    pub fn default_call() -> CanvasContent {
        CanvasContent::default()
    }
}

impl Default for CanvasContent {
    fn default() -> Self {
        let mut vgc_data = vgc::generate_from_line(vec![
            vec![
                Coord::new(0.0 * 1.5, 0.1),
                Coord::new(0.0 * 1.5, 1.0),
                Coord::new(0.9 * 1.5, 1.0),
            ],
            vec![
                Coord::new(1.0 * 1.5, 0.9),
                Coord::new(0.1 * 1.5, 0.0),
                Coord::new(1.0 * 1.5, 0.0),
            ],
        ]);

        let shape = vgc_data.get_shape_mut(0).expect("Valid");
        shape.color.r = 128;
        shape.color.g = 0;

        vgc_data.max_size = -1.5;
        let camera = Camera::new(vgc_data.max_rect().center(), 500.0);
        Self { camera, vgc_data }
    }
}

#[wasm_bindgen]
pub fn render(
    ctx: &CanvasRenderingContext2d,
    canvas_content: &CanvasContent,
) -> Result<(), JsValue> {
    let vgc = &canvas_content.vgc_data;

    let transform = canvas_content.camera.get_transform();

    let mut ctx_2d_renderer = CanvasContext2DRender::new(
        ctx,
        (transform.0 as f64, transform.1 as f64),
        transform.2 as f64,
        transform.3 as f64,
    );

    let pixel_region = canvas_content.camera.get_pixel_region();

    ctx.clear_rect(
        pixel_region.top_left.c.x as f64,
        pixel_region.top_left.c.y as f64,
        pixel_region.width() as f64,
        pixel_region.height() as f64,
    );
    let result = vgc.render(&mut ctx_2d_renderer);
    match result {
        Err(string) => {
            return Err(JsValue::from_str(&string));
        }
        _ => {}
    };

    Ok(())
}

#[wasm_bindgen]
pub fn render_cover(
    ctx: &CanvasRenderingContext2d,
    canvas_content: &CanvasContent,
    width: f64,
    height: f64,
) -> Result<(), JsValue> {
    console_log!("render_full width: {}, height: {}", width, height);
    let vgc = &canvas_content.vgc_data;

    let max_rect = vgc.max_rect();

    let scale_x = width / max_rect.width() as f64;
    let scale_y = height / max_rect.height() as f64;

    let mut ctx_2d_renderer = CanvasContext2DRender::new(ctx, (0.0, 0.0), scale_x, scale_y);

    let result = vgc.render(&mut ctx_2d_renderer);
    match result {
        Err(string) => {
            return Err(JsValue::from_str(&string));
        }
        _ => {}
    };

    Ok(())
}

//------------------------------------------------------------------------------
// Utilities
//------------------------------------------------------------------------------

/// Panic hook lets us get better error messages if our Rust code ever panics.
///
/// For more details see
/// <https://github.com/rustwasm/console_error_panic_hook#readme>
#[wasm_bindgen(js_name = "setPanicHook")]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    // For alerting
    pub(crate) fn alert(s: &str);
    // For logging in the console.
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
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

/// Calls the wasm_bindgen console.log.
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => ($crate::log(&format_args!($($t)*).to_string()))
}
