use super::Command;
use crate::{scene::shape::boolean::ShapeUnion, LayerId, Shape};
use anyhow::{Ok, Result};
use macros::boxed;
use std::any::Any;

#[derive(Clone, Debug)]
pub struct Union {
    a: LayerId,
    b: LayerId,
    a_shape: Option<Shape>,
    b_shape: Option<Shape>,
    b_shape_pos: Option<usize>,
    result: Option<ShapeUnion>,
}

impl Union {
    #[boxed]
    pub fn new(a: LayerId, b: LayerId) -> Self {
        Self {
            a,
            b,
            a_shape: None,
            b_shape: None,
            result: None,
            b_shape_pos: None,
        }
    }
}

impl Command for Union {
    fn execute(&mut self, scene: &mut crate::Scene) -> Result<()> {
        let res = {
            let a_shape = scene.shape_select(self.a).expect("not 404");
            let b_shape = scene.shape_select(self.b).expect("not 404");

            self.a_shape = Some(a_shape.clone());
            self.b_shape = Some(b_shape.clone());
            a_shape.union(&b_shape)
        };
        self.result = Some(res.clone());

        self.b_shape_pos = Some(scene.layer_position(self.b).expect("not 404"));
        match res {
            ShapeUnion::A => {
                scene.layer_delete(self.b);
            }
            ShapeUnion::B => {
                scene.layer_delete(self.b);

                let a_shape = scene.shape_select_mut(self.a).expect("not 404");
                a_shape.path = self.b_shape.as_ref().unwrap().path.clone();
            }
            ShapeUnion::New(shape) => {
                scene.layer_delete(self.b);

                let a_shape = scene.shape_select_mut(self.a).expect("not 404");
                a_shape.path = shape.path;
            }
            ShapeUnion::None => {
                self.b_shape_pos = None;
            }
        };

        Ok(())
    }

    fn undo(&mut self, scene: &mut crate::Scene) -> Result<()> {
        let a_shape_undo = self.a_shape.take().expect("No shape to undo");
        let b_shape = self.b_shape.take().expect("No shape to undo");
        let result = self.result.take().expect("No result to undo");

        match result {
            ShapeUnion::A => {
                let id = scene.shape_insert(b_shape);
                scene
                    .layer_move_at(id, self.b_shape_pos.unwrap())
                    .expect("not 404");
            }
            ShapeUnion::B => {
                let id = scene.shape_insert(b_shape);
                scene
                    .layer_move_at(id, self.b_shape_pos.unwrap())
                    .expect("not 404");

                let a_shape = scene.shape_select_mut(self.a).expect("not 404");
                a_shape.path = a_shape_undo.path;
            }
            ShapeUnion::New(_) => {
                let id = scene.shape_insert(b_shape);
                scene
                    .layer_move_at(id, self.b_shape_pos.unwrap())
                    .expect("not 404");

                let a_shape = scene.shape_select_mut(self.a).expect("not 404");
                a_shape.path = a_shape_undo.path;
            }
            ShapeUnion::None => {}
        };

        Ok(())
    }

    fn merge(&self, _: &dyn Command) -> Option<Result<Box<dyn Command>>> {
        // 
        // match self.result.as_ref().unwrap() {
        //     ShapeUnion::None => {
        //         return Some(Ok(Box::new(other.clone())));
        //     }
        //     _ => {}
        // }
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

    use super::Union;

    #[test]
    fn given_square_when_move_coords() {
        let mut scene = Scene::new();
        let a = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.4, 0.4));
        let a_id = scene.shape_insert(a);

        let b = Shape::new_circle(Coord::new(0.5, 0.0), Length2d::new(0.4, 0.4));
        let b_id = scene.shape_insert(b);

        let expected_scene = scene.clone();

        let mut command = Union::new(a_id, b_id);
        command.execute(&mut scene).unwrap();

        let new_shape = scene.shape_select(a_id).unwrap();
        assert_eq!(new_shape.path.len(), 25);

        command.undo(&mut scene).unwrap();

        assert_eq!(expected_scene, scene);
    }
}
