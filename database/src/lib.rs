mod math;
mod scene;
mod user_context;

pub mod commands;
#[cfg(test)]
mod integration;

pub use scene::id::CoordId;
pub use scene::id::LayerId;
pub use scene::render::DrawingContext;
pub use scene::render::RenderOption;
pub use scene::shape::coord::DbCoord;
pub use scene::shape::curve::Curve;
pub use scene::shape::Shape;
pub use scene::tree_view::TreeViewModel;
pub use scene::Scene;

pub use user_context::user_selection::SelectedLevel;
pub use user_context::user_selection::UserSelection;
pub use user_context::SceneUserContext;

/// Maximum size of the image, if we want to have detail for each pixel
/// This is a limit because of f32 precision with 2^-23 for the smallest value
/// See decision.md for more information
#[allow(dead_code)]
static MAX_DETAIL_SIZE: u32 = 52000000;
