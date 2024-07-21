mod fill;
mod macros;
pub mod math;
pub mod pures;
pub mod types;

pub use fill::Rgba;

const POINT_PER_PIXEL: f32 = 50.0;
pub const PRECISION: f32 = f32::EPSILON * POINT_PER_PIXEL;

//macro for debug string appending file! and line! to a string
#[macro_export]
macro_rules! dbg_str {
    ($($arg:tt)*) => {
        format!("{}:{}: {}", file!(), line!(), format!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dbg_str() {
        let b = dbg_str!("test");
        assert_eq!(b, "common\\src\\lib.rs:22: test");
    }
}
