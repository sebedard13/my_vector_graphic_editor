pub mod api;
mod camera;
mod canvas_context_2d_render;
pub mod user_selection;

use crate::canvas_context_2d_render::CanvasContext2DRender;
use camera::Camera;
use vgc::{coord::Coord, Vgc};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::CanvasRenderingContext2d;

pub use vgc::Rgba;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

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
        let mut vgc_data = vgc::generate_from_line(vec![vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 0.0, y: 1.0 },
            Coord { x: 1.0, y: 1.0 },
            Coord { x: 1.0, y: 0.0 },
        ]]);

        let shape = vgc_data.get_shape_mut(0).expect("Valid");
        shape.color.r = 255;
        shape.color.g = 255;
        shape.color.b = 255;

        vgc_data.ratio = (width / height) as f64;
        Self {
            camera: Camera::new(vgc_data.ratio as f32),
            vgc_data,
        }
    }

    #[wasm_bindgen]
    pub fn default_call() -> CanvasContent {
        CanvasContent::default()
    }

    pub fn set_pixel_region(&mut self, width: f32, height: f32) {
        self.camera.pixel_region = (0.0, 0.0, width, height);
    }

    pub fn get_project_mouse(&self, x: f32, y: f32) -> Point {
        let (x, y) = self.camera.project((x, y));
        Point { x, y }
    }

    pub fn get_zoom(&self) -> f32 {
        self.camera.scaling
    }

    ///  Zooms the camera in or out by the given amount, centered on the given point.
    ///
    ///  # Arguments
    ///  * `movement` - positive for zoom in, negative for zoom out. Only 1 or -1 are supported.
    ///  * `x` - x coordinate of the center of the zoom
    ///  * `y` - y coordinate of the center of the zoom
    pub fn zoom(&mut self, movement: f32, x: f32, y: f32) {
        self.camera.handle_zoom(movement, (x, y));
    }

    pub fn pan_camera(&mut self, x: f32, y: f32) {
        self.camera.handle_pan((x, y));
    }

    pub fn get_ratio(&self) -> f64 {
        self.vgc_data.ratio
    }
}

impl Default for CanvasContent {
    fn default() -> Self {
        let mut vgc_data = vgc::generate_from_line(vec![
            vec![
                Coord { x: 0.0, y: 0.1 },
                Coord { x: 0.0, y: 1.0 },
                Coord { x: 0.9, y: 1.0 },
            ],
            vec![
                Coord { x: 1.0, y: 0.9 },
                Coord { x: 0.1, y: 0.0 },
                Coord { x: 1.0, y: 0.0 },
            ],
        ]);

        let shape = vgc_data.get_shape_mut(0).expect("Valid");
        shape.color.r = 128;
        shape.color.g = 0;

        vgc_data.ratio = 1.5;
        Self {
            camera: Camera::new(vgc_data.ratio as f32),
            vgc_data
        }
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

    ctx.clear_rect(
        canvas_content.camera.pixel_region.0 as f64,
        canvas_content.camera.pixel_region.1 as f64,
        canvas_content.camera.pixel_region.2 as f64,
        canvas_content.camera.pixel_region.3 as f64,
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
pub fn render_full(
    ctx: &CanvasRenderingContext2d,
    canvas_content: &CanvasContent,
    width: f64,
    height: f64,
) -> Result<(), JsValue> {
    let vgc = &canvas_content.vgc_data;

    let mut ctx_2d_renderer = CanvasContext2DRender::new(
        ctx,
        (0.0, 0.0),
        width,
        height,
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
