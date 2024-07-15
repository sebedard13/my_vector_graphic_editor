mod math;
mod scene;
mod user_context;

#[cfg(test)]
mod integration;

pub use scene::id::CoordId;
pub use scene::id::LayerId;
pub use scene::render::DrawingContext;
pub use scene::shape::coord::DbCoord;
pub use scene::shape::curve::Curve;
pub use scene::shape::Shape;
pub use scene::Scene;

pub use user_context::user_selection::SelectedLevel;
pub use user_context::user_selection::UserSelection;
pub use user_context::SceneUserContext;
