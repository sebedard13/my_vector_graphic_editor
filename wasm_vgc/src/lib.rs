use js_sys::Uint8ClampedArray;
use vgc::{TinySkiaRenderer, coord::Coord};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn render() -> Uint8ClampedArray {
    console_log!("Render");
    let mut tiny_skia_renderer = TinySkiaRenderer::new();
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
    let width = 512;
    let result = vgc.render_w(&mut tiny_skia_renderer,512);

    match result{
        Err(string) => {
            console_log!("{}", string);
            return js_sys::Uint8ClampedArray::from((vec![0 as u8; width*width]).as_slice())
        },
        _ =>{},
    }
    js_sys::Uint8ClampedArray::from(tiny_skia_renderer.get_rgba().expect("valid after match result").as_slice())
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
