mod canvas_context_2d_render;

use vgc::coord::Coord;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::CanvasRenderingContext2d;
use crate::canvas_context_2d_render::CanvasContext2DRender;

#[wasm_bindgen]
pub fn render(ctx: &CanvasRenderingContext2d, width: usize, height: usize) -> Result<(), JsValue> {
   
    let vgc = vgc::generate_from_line(vec![
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


    let vgc_width = 512;
    let vgc_height = ((vgc_width as f64) * (1.0 / vgc.ratio)) as u32;
   

    let coord_center = (width as f64 / 2.0, height as f64 / 2.0);

    let top_left = (
        coord_center.0 - (vgc_width as f64 / 2.0),
        coord_center.1 - (vgc_height as f64 / 2.0),
    );

    let mut ctx_2d_renderer = CanvasContext2DRender::new(ctx, top_left);

    let result = vgc.render_w(&mut ctx_2d_renderer, vgc_width);
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
