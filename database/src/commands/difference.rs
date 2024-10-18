use super::Command;
use crate::{scene::shape::boolean::ShapeDifference, LayerId, Shape};
use anyhow::{Ok, Result};
use log::warn;
use macros::boxed;
use std::any::Any;

#[derive(Clone, Debug)]
pub struct Difference {
    a: LayerId,
    b: LayerId,
    a_shape: Option<Shape>,
    b_shape: Option<Shape>,
    a_shape_pos: Option<usize>,
    b_shape_pos: Option<usize>,
    result: Option<ShapeDifference>,
    new_shapes: Option<Vec<LayerId>>,
}

impl Difference {
    #[boxed]
    pub fn new(a: LayerId, b: LayerId) -> Self {
        Self {
            a,
            b,
            a_shape: None,
            b_shape: None,
            result: None,
            a_shape_pos: None,
            b_shape_pos: None,
            new_shapes: None,
        }
    }
}

impl Command for Difference {
    fn execute(&mut self, scene: &mut crate::Scene) -> Result<()> {
        let res = {
            let a_shape = scene.shape_select(self.a).expect("not 404");
            let b_shape = scene.shape_select(self.b).expect("not 404");

            self.a_shape = Some(a_shape.clone());
            self.b_shape = Some(b_shape.clone());
            a_shape.difference(&b_shape)
        };
        self.result = Some(res.clone());

        self.b_shape_pos = Some(scene.layer_position(self.b).expect("not 404"));
        match res {
            ShapeDifference::A => {
                scene.layer_delete(self.b);
            }
            ShapeDifference::EraseA => {
                scene.layer_delete(self.b);
                self.a_shape_pos = Some(scene.layer_position(self.a).expect("not 404"));
                scene.layer_delete(self.a);
            }
            ShapeDifference::New(mut shapes) => {
                scene.layer_delete(self.b);

                let a_shape = scene.shape_select_mut(self.a).expect("not 404");
                a_shape.path = shapes.remove(0).path;

                if shapes.len() > 0 {
                    let mut new_shapes = Vec::new();
                    for shape in shapes {
                        let id = scene.shape_insert(shape);
                        new_shapes.push(id);
                    }
                    self.new_shapes = Some(new_shapes);
                }
            }
            ShapeDifference::AWithBHole => {
                warn!("A with B hole not implemented");
            }
        };

        Ok(())
    }

    fn undo(&mut self, scene: &mut crate::Scene) -> Result<()> {
        let a_shape_undo = self.a_shape.take().expect("No shape to undo");
        let b_shape = self.b_shape.take().expect("No shape to undo");
        let result = self.result.take().expect("No result to undo");

        match result {
            ShapeDifference::A => {
                let id = scene.shape_insert(b_shape);
                scene
                    .layer_move_at(id, self.b_shape_pos.unwrap())
                    .expect("not 404");
            }
            ShapeDifference::EraseA => {
                let id = scene.shape_insert(a_shape_undo);
                scene
                    .layer_move_at(id, self.a_shape_pos.unwrap())
                    .expect("not 404");

                let id = scene.shape_insert(b_shape);
                scene
                    .layer_move_at(id, self.b_shape_pos.unwrap())
                    .expect("not 404");
            }
            ShapeDifference::New(_) => {
                if let Some(new_shapes) = self.new_shapes.take() {
                    for id in new_shapes {
                        scene.layer_delete(id);
                    }
                }

                let a_shape = scene.shape_select_mut(self.a).expect("not 404");
                a_shape.path = a_shape_undo.path;

                let id = scene.shape_insert(b_shape);
                scene
                    .layer_move_at(id, self.b_shape_pos.unwrap())
                    .expect("not 404");
            }
            ShapeDifference::AWithBHole => {}
        };

        Ok(())
    }

    fn merge(&self, _: &dyn Command) -> Option<Result<Box<dyn Command>>> {
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod test {
    use crate::{commands::Command, Scene, Shape};
    use common::types::{Coord, Length2d};

    use super::Difference;

    #[test]
    fn given_circles_when_difference_undo() {
        let mut scene = Scene::new();
        let a = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.4, 0.4));
        let a_id = scene.shape_insert(a);

        let b = Shape::new_circle(Coord::new(0.5, 0.0), Length2d::new(0.4, 0.4));
        let b_id = scene.shape_insert(b);

        let expected_scene = scene.clone();

        let mut command = Difference::new(a_id, b_id);
        command.execute(&mut scene).unwrap();

        let new_shape = scene.shape_select(a_id).unwrap();
        assert_eq!(new_shape.path.len(), 19);

        command.undo(&mut scene).unwrap();

        assert_eq!(expected_scene, scene);
    }
}
