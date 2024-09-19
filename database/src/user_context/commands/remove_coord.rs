#![allow(dead_code)]
use super::Command;
use crate::{CoordId, LayerId, Shape};
use anyhow::{Context, Error, Ok, Result};
use std::any::Any;

#[derive(Clone)]
pub struct RemoveCoords {
    shape: LayerId,
    coord: CoordId,

    shape_undo: Option<Shape>,
    //Will be none if the shape is not deleted
    shape_position: Option<usize>,
}

impl RemoveCoords {
    pub fn new(shape: LayerId, coord: CoordId) -> Self {
        Self {
            shape,
            coord,
            shape_undo: None,
            shape_position: None,
        }
    }
}

impl Command for RemoveCoords {
    fn execute(&mut self, scene: &mut crate::Scene) -> Result<()> {
        let shape = scene.shape_select_mut(self.shape).expect("not 404");
        self.shape_undo = Some(shape.clone());
        shape.coord_delete(self.coord).expect("not 404");

        if shape.is_empty() {
            self.shape_position = Some(scene.layer_position(self.shape).expect("not 404"));
            scene.layer_delete(self.shape);
            //Todo user selection need to be updated
        }

        Ok(())
    }

    fn undo(&mut self, scene: &mut crate::Scene) -> Result<()> {
        let shape = self.shape_undo.take().context("No shape to undo")?;
        if let Some(index) = self.shape_position.take() {
            let id = scene.shape_insert(shape);
            scene.layer_move_at(id, index).map_err(Error::msg)?;
            Ok(())
        } else {
            scene.shape_put(shape);
            Ok(())
        }
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
    use common::types::{Coord, Length2d};

    use crate::{user_context::commands::CommandsHandler, Scene};

    use super::*;

    #[test]
    fn given_close_shape_when_remove_any_coord() {
        let len = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.5, 0.5))
            .path
            .len();

        for i in 0..len {
            let mut scene = Scene::new();
            let shape = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.5, 0.5));

            let coord_id = shape.path[i].id;
            let coord = shape.path[i].coord();

            let shape_id = scene.shape_insert(shape);
            let expected = scene.clone();

            let mut command = RemoveCoords::new(shape_id, coord_id);
            command.execute(&mut scene).unwrap();

            let new_shape = scene.shape_select(shape_id).unwrap();
            for c in new_shape.path.iter() {
                assert_ne!(c.id, coord_id);
                assert_ne!(c.coord(), coord);
            }

            command.undo(&mut scene).unwrap();

            assert_eq!(expected, scene);
        }
    }

    #[test]
    fn given_close_shape_when_remove_all_coords_then_undo() {
        let mut scene = Scene::new();
        let shape = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.5, 0.5));

        let shape_id = scene.shape_insert(shape);

        let expected = scene.clone();
        let mut commands_handler = CommandsHandler::from(scene);

        let index_to_remove = vec![6,3,0,0];
        for i in index_to_remove {
            let coord_id = commands_handler.scene().shape_select(shape_id).unwrap().path[i].id;

            let command = RemoveCoords::new(shape_id, coord_id);
            commands_handler.execute(Box::new(command)).unwrap();
        }

        assert!(commands_handler.scene().shape_select(shape_id).is_none());
        commands_handler.undo().unwrap();
        commands_handler.undo().unwrap();
        commands_handler.undo().unwrap();
        commands_handler.undo().unwrap();

        assert_eq!(expected, *commands_handler.scene());
    }

}
