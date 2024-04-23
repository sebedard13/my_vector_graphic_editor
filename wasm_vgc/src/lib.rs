pub mod api;
mod camera;
mod canvas_context_2d_render;

pub mod user_selection;

use crate::canvas_context_2d_render::CanvasContext2DRender;
use camera::Camera;
use common::{
    dbg_str,
    pures::{Affine, Vec2},
    types::{Coord, ScreenRect},
    Rgba,
};
use log::{info, warn};
use vgc::{coord::coordptr_new, shape::Shape, Vgc};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::CanvasRenderingContext2d;

pub use common;

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
            Coord::new(-1.0, -1.0),
            Coord::new(-1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, -1.0),
        ]]);

        let shape = vgc_data.get_shape_mut(0).expect("Valid");
        shape.color.r = 255;
        shape.color.g = 255;
        shape.color.b = 255;

        let camera = Camera::new(vgc_data.max_rect().center(), width, height);

        Self { camera, vgc_data }
    }

    pub fn get_render_rect(&self) -> ScreenRect {
        let size = self.camera.get_base_scale();

        let sc = ScreenRect::new(0.0, 0.0, size.c.x, size.c.y);
        sc
    }

    #[wasm_bindgen]
    pub fn default_call() -> CanvasContent {
        CanvasContent::default()
    }

    pub fn debug_string(&self) -> String {
        self.vgc_data.debug_string()
    }
}

impl Default for CanvasContent {
    fn default() -> Self {
        let mut vgc_data = Vgc::new(Rgba::white());

        let c0 = coordptr_new(-1.0, -1.0);

        let c2 = coordptr_new(1.0, 1.0);
        let c3 = coordptr_new(0.0, 0.0);

        let mut shape0 = Shape::new(Coord::new(-1.0, 1.0), Rgba::new(128, 0, 0, 255));
        let mut shape1 = Shape::new(Coord::new(1.0, -1.0), Rgba::new(0, 0, 0, 255));

        shape0.push_coord(shape0.start.clone(), c0.clone(), c0.clone());
        shape0.push_coord(c0.clone(), c3.clone(), c3.clone());
        shape0.push_coord(c3.clone(), c2.clone(), c2.clone());
        shape0.push_coord(c2.clone(), shape0.start.clone(), shape0.start.clone());

        shape1.push_coord(shape1.start.clone(), c2.clone(), c2.clone());
        shape1.push_coord(c2.clone(), c3.clone(), c3.clone());
        shape1.push_coord(c3.clone(), c0.clone(), c0.clone());
        shape1.push_coord(c0.clone(), shape1.start.clone(), shape1.start.clone());

        vgc_data.push_shape(shape0);
        vgc_data.push_shape(shape1);

        let camera = Camera::new(vgc_data.max_rect().center(), 750.0, 500.0);
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

    let mut ctx_2d_renderer = CanvasContext2DRender::new(ctx, transform);

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
    width: f32,
    height: f32,
) -> Result<(), JsValue> {
    let vgc = &canvas_content.vgc_data;

    let max_rect = vgc.max_rect();

    let scale_x = width / max_rect.width();
    let scale_y = height / max_rect.height();

    let mut ctx_2d_renderer = CanvasContext2DRender::new(
        ctx,
        Affine::identity()
            .translate(max_rect.top_left.c * -1.0)
            .scale(Vec2::new(scale_x, scale_y)),
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
