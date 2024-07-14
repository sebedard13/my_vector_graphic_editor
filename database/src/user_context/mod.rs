use camera::Camera;
use common::pures::Affine;

use crate::{scene::shape, DbCoord, DrawingContext, Scene, Shape};

pub mod api;
pub mod camera;
pub mod user_selection;

pub struct SceneUserContext {
    pub scene: Scene,
    pub camera: Camera,
}

impl SceneUserContext {
    pub fn new() -> Self {
        let scene = Scene::new();
        let camera = Camera::new(scene.max_rect().center(), 500.0, 500.0);
        Self { scene, camera }
    }

    pub fn scene_render<T: DrawingContext>(&self, drawing_context: &mut T) -> Result<(), String> {
        self.scene.render(drawing_context)
    }
}

impl Default for SceneUserContext {
    fn default() -> Self {
        let mut scene = Scene::new();
        let mut shape1 = Shape::new_from_path(
            &vec![
                DbCoord::new(-1.0, -0.9),
                DbCoord::new(-1.0, 1.0),
                DbCoord::new(0.9, 1.0),
            ],
            Affine::identity(),
        );
        shape1.color.r = 128;
        shape1.color.g = 0;
        scene.shape_insert(shape1);

        let shape2 = Shape::new_from_path(
            &vec![
                DbCoord::new(1.0, 0.9),
                DbCoord::new(-0.9, -1.0),
                DbCoord::new(1.0, -1.0),
            ],
            Affine::identity(),
        );
        scene.shape_insert(shape2);

        let  camera = Camera::new(scene.max_rect().center(), 750.0, 500.0);
        Self { scene, camera }
    }
}
