use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
#[wasm_bindgen]
impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba { r, g, b, a }
    }

    ///
    /// ```rust
    /// use vgc::Rgba;
    /// let rgba = Rgba::new(255, 255, 255, 255);
    /// assert_eq!(rgba.to_css_string(), "rgba(255,255,255,255)");
    ///
    /// let rgba = Rgba::new(25, 50, 75, 100);
    /// assert_eq!(rgba.to_css_string(), "rgba(25,50,75,100)");
    /// ```
    pub fn to_css_string(&self) -> String {
        return format!("rgba({},{},{},{})", self.r, self.g, self.b, self.a);
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
