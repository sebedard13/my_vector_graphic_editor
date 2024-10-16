use crate::{commands::CommandsHandler, scene::shape::Stroke};
use camera::Camera;
use common::{pures::Affine, Rgba};
use serde::{Deserialize, Serialize};

use crate::{DbCoord, DrawingContext, RenderOption, Scene, Shape};

pub mod api;
pub mod camera;

mod ui;
mod boolean;
pub mod user_selection;

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneUserContext {
    pub command_handler: CommandsHandler,
    pub render_options: RenderOption,
    pub camera: Camera,
}

impl SceneUserContext {
    pub fn new(width: f32, height: f32) -> Self {
        let scene = Scene::new();
        let camera = Camera::new(scene.max_rect().center(), width, height);
        let render_options = RenderOption::default();
        Self {
            command_handler: CommandsHandler::from(scene),
            render_options,
            camera,
        }
    }

    pub fn scene(&self) -> &Scene {
        &self.command_handler.scene()
    }

    pub fn scene_render<T: DrawingContext>(&self, drawing_context: &mut T) -> Result<(), String> {
        self.scene()
            .render_with_options(drawing_context, self.render_options.clone())
    }
}

impl Default for SceneUserContext {
    fn default() -> Self {
        let mut scene = Scene::new();
        let mut shape1 = Shape::new_from_lines(
            vec![
                DbCoord::new(-1.0, -0.9),
                DbCoord::new(-1.0, 1.0),
                DbCoord::new(0.9, 1.0),
            ],
            Affine::identity(),
        );
        shape1.color = Rgba::new(128, 0, 0, 255);
        shape1.stroke = Stroke::new(0.02, Rgba::new(0, 255, 0, 255));
        scene.shape_insert(shape1);

        let mut shape2 = Shape::new_from_lines(
            vec![
                DbCoord::new(1.0, 0.9),
                DbCoord::new(-0.9, -1.0),
                DbCoord::new(1.0, -1.0),
            ],
            Affine::identity(),
        );
        shape2.color = Rgba::new(0, 0, 0, 255);
        scene.shape_insert(shape2);

        let camera = Camera::new(scene.max_rect().center(), 750.0, 500.0);
        let render_options = RenderOption::default();
        Self {
            command_handler: CommandsHandler::from(scene),
            render_options,
            camera,
        }
    }
}
