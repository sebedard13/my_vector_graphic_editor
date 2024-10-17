use super::Command;
use crate::{scene::shape::boolean::ShapeIntersection, LayerId, Shape};
use anyhow::{Ok, Result};
use macros::boxed;
use std::any::Any;

#[derive(Clone, Debug)]
pub struct Intersection {
    a: LayerId,
    b: LayerId,
    a_shape: Option<Shape>,
    b_shape: Option<Shape>,
    a_shape_pos: Option<usize>,
    b_shape_pos: Option<usize>,
    result: Option<ShapeIntersection>,
    new_shapes: Option<Vec<LayerId>>,
}

impl Intersection {
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

impl Command for Intersection {
    fn execute(&mut self, scene: &mut crate::Scene) -> Result<()> {
        let res = {
            let a_shape = scene.shape_select(self.a).expect("not 404");
            let b_shape = scene.shape_select(self.b).expect("not 404");

            self.a_shape = Some(a_shape.clone());
            self.b_shape = Some(b_shape.clone());
            a_shape.intersection(&b_shape)
        };
        self.result = Some(res.clone());

        self.b_shape_pos = Some(scene.layer_position(self.b).expect("not 404"));
        match res {
            ShapeIntersection::A => {
                scene.layer_delete(self.b);
            }
            ShapeIntersection::B => {
                scene.layer_delete(self.b);

                let a_shape = scene.shape_select_mut(self.a).expect("not 404");
                a_shape.path = self.b_shape.as_ref().unwrap().path.clone();
            }
            ShapeIntersection::New(mut shapes) => {
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
            ShapeIntersection::None => {
                scene.layer_delete(self.b);
                scene.layer_delete(self.a);
            }
        };

        Ok(())
    }

    fn undo(&mut self, scene: &mut crate::Scene) -> Result<()> {
        let a_shape_undo = self.a_shape.take().expect("No shape to undo");
        let b_shape = self.b_shape.take().expect("No shape to undo");
        let result = self.result.take().expect("No result to undo");

        match result {
            ShapeIntersection::A => {
                let id = scene.shape_insert(b_shape);
                scene
                    .layer_move_at(id, self.b_shape_pos.unwrap())
                    .expect("not 404");
            }
            ShapeIntersection::B => {
                let id = scene.shape_insert(b_shape);
                scene
                    .layer_move_at(id, self.b_shape_pos.unwrap())
                    .expect("not 404");

                let a_shape = scene.shape_select_mut(self.a).expect("not 404");
                a_shape.path = a_shape_undo.path;
            }
            ShapeIntersection::New(_) => {
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
            ShapeIntersection::None => {
                scene.shape_insert(a_shape_undo);
                scene
                    .layer_move_at(self.a, self.a_shape_pos.unwrap())
                    .expect("not 404");

                let id = scene.shape_insert(b_shape);
                scene
                    .layer_move_at(id, self.b_shape_pos.unwrap())
                    .expect("not 404");
            }
        }

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

    use super::Intersection;

    #[test]
    fn given_circles_when_difference_undo() {
        let mut scene = Scene::new();
        let a = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.4, 0.4));
        let a_id = scene.shape_insert(a);

        let b = Shape::new_circle(Coord::new(0.5, 0.0), Length2d::new(0.4, 0.4));
        let b_id = scene.shape_insert(b);

        let expected_scene = scene.clone();

        let mut command = Intersection::new(a_id, b_id);
        command.execute(&mut scene).unwrap();

        let new_shape = scene.shape_select(a_id).unwrap();
        assert_eq!(new_shape.path(), "M 0.25000012 0.3122315 C 0.34133744 0.23904903 0.3997203 0.1265963 0.4000221 0 C 0.3997203 -0.1265963 0.34133744 -0.23904905 0.2500001 -0.31223154 C 0.15866256 -0.23904903 0.10027971 -0.1265963 0.09997791 0 C 0.10027971 0.1265963 0.15866256 0.23904905 0.25000012 0.3122315 Z");
        assert_eq!(new_shape.path.len(), 13);

        command.undo(&mut scene).unwrap();

        assert_eq!(expected_scene, scene);
    }
}
