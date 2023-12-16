use std::mem;

use js_sys::Uint8ClampedArray;
use vgc::{TinySkiaRenderer, coord::Coord};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn render(width: usize, height: usize) -> Uint8ClampedArray {
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

    let mut main_background_u32 = vec![0xff020202 as u32; width * height];

    let result = vgc.render_w(&mut tiny_skia_renderer,512);

    match result{
        Err(string) => {
            console_log!("{}", string);
            return js_sys::Uint8ClampedArray::from((vec![0 as u8; width*width]).as_slice())
        },
        _ =>{},
    }

    // I copy-pasted this code from StackOverflow without reading the answer 
    // surrounding it that told me to write a comment explaining why this code 
    // is actually safe for my own use case.
    let vec8 = unsafe {
        let ratio = mem::size_of::<u32>() / mem::size_of::<u8>();

        let length = main_background_u32.len() * ratio;
        let capacity = main_background_u32.capacity() * ratio;
        let ptr = main_background_u32.as_mut_ptr() as *mut u8;

        // Don't run the destructor for vec32
        mem::forget(main_background_u32);

        // Construct new Vec
        Vec::from_raw_parts(ptr, length, capacity)
    };
    js_sys::Uint8ClampedArray::from(vec8.as_slice())
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
