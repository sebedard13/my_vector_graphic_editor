use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba { r, g, b, a }
    }

    pub fn black() -> Rgba {
        Rgba::new(0, 0, 0, 255)
    }

    pub fn white() -> Rgba {
        Rgba::new(255, 255, 255, 255)
    }

    pub fn transparent() -> Rgba {
        Rgba::new(0, 0, 0, 0)
    }

    ///
    /// ```rust
    /// use common::Rgba;
    /// let rgba = Rgba::new(255, 255, 255, 255);
    /// assert_eq!(rgba.to_css_string(), "rgba(255,255,255,255)");
    ///
    /// let rgba = Rgba::new(25, 50, 75, 100);
    /// assert_eq!(rgba.to_css_string(), "rgba(25,50,75,100)");
    /// ```
    pub fn to_css_string(&self) -> String {
        format!("rgba({},{},{},{})", self.r, self.g, self.b, self.a)
    }

    ///```rust
    /// use common::Rgba;
    /// let rgba = Rgba::new(255, 255, 255, 255);
    /// assert_eq!(rgba.to_hex_string(), "#ffffffff");
    ///
    /// let rgba = Rgba::new(25, 50, 75, 100);
    /// assert_eq!(rgba.to_hex_string(), "#19324b64");
    pub fn to_hex_string(&self) -> String {
        format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
    }

    ///```rust
    /// use common::Rgba;
    /// let rgba = Rgba::new(255, 255, 255, 255);
    /// assert_eq!(rgba.to_small_hex_string(), "#ffffff");
    ///
    /// let rgba = Rgba::new(25, 50, 75, 100);
    /// assert_eq!(rgba.to_small_hex_string(), "#19324b");
    pub fn to_small_hex_string(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn from_small_hex_string(hex: &str) -> Rgba {
        let result = Rgba::from_small_hex_safe(hex);
        match result {
            Ok(rgba) => rgba,
            Err(_) => {
                //Todo Log error
                Rgba::new(0, 0, 0, 255)
            }
        }
    }

    pub fn from_small_hex_safe(hex: &str) -> Result<Rgba, String> {
        let hex = hex.trim_start_matches('#');
        let mut hex = hex.chars();

        let r = hex
            .next()
            .ok_or("Invalid hex string")?
            .to_digit(16)
            .ok_or("Invalid hex string")? as u8;
        let r = r * 16
            + hex
                .next()
                .ok_or("Invalid hex string")?
                .to_digit(16)
                .ok_or("Invalid hex string")? as u8;

        let g = hex
            .next()
            .ok_or("Invalid hex string")?
            .to_digit(16)
            .ok_or("Invalid hex string")? as u8;
        let g = g * 16
            + hex
                .next()
                .ok_or("Invalid hex string")?
                .to_digit(16)
                .ok_or("Invalid hex string")? as u8;

        let b = hex
            .next()
            .ok_or("Invalid hex string")?
            .to_digit(16)
            .ok_or("Invalid hex string")? as u8;
        let b = b * 16
            + hex
                .next()
                .ok_or("Invalid hex string")?
                .to_digit(16)
                .ok_or("Invalid hex string")? as u8;

        Ok(Rgba::new(r, g, b, 255))
    }
}

impl From<[u8; 4]> for Rgba {
    fn from(value: [u8; 4]) -> Self {
        Rgba {
            r: value[0],
            g: value[1],
            b: value[2],
            a: value[3],
        }
    }
}

impl Default for Rgba {
    fn default() -> Self {
        Rgba::black()
    }
}

use wasm_bindgen::convert::*;
use wasm_bindgen::describe::*;

impl WasmDescribeVector for Rgba {
    fn describe_vector() {
        inform(VECTOR);
        Rgba::describe();
    }
}

impl VectorIntoWasmAbi for Rgba {
    type Abi = <
        wasm_bindgen::__rt::std::boxed::Box<[wasm_bindgen::JsValue]>
        as wasm_bindgen::convert::IntoWasmAbi
        >::Abi;

    fn vector_into_abi(vector: wasm_bindgen::__rt::std::boxed::Box<[Rgba]>) -> Self::Abi {
        wasm_bindgen::convert::js_value_vector_into_abi(vector)
    }
}
