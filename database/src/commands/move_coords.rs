use super::Command;
use crate::{CoordId, LayerId};
use anyhow::{Ok, Result};
use common::types::Coord;
use macros::boxed;
use std::any::Any;

#[derive(Clone, Debug)]
pub struct MoveCoords {
    selection: Vec<(LayerId, Vec<CoordId>)>,
    start_pos: Coord,
    end_pos: Coord,
}

impl MoveCoords {
    #[boxed]
    pub fn new(selection: Vec<(LayerId, Vec<CoordId>)>, start_pos: Coord, end_pos: Coord) -> Self {
        Self {
            selection,
            start_pos,
            end_pos,
        }
    }
}

impl Command for MoveCoords {
    fn execute(&mut self, scene: &mut crate::Scene) -> Result<()> {
        for shape_selection in self.selection.iter() {
            let shape = scene.shape_select_mut(shape_selection.0).expect("not 404");
            for coord_id in &shape_selection.1 {
                let (coord_id, coord) = {
                    let coord = shape.coord_select_mut(*coord_id).expect("not 404");
                    let movement = self.end_pos - self.start_pos;
                    let res_vec2 = coord.coord() + movement;
                    (coord.id, Coord::new(res_vec2.x, res_vec2.y))
                };

                shape.coord_set(coord_id, coord);
            }
        }
        Ok(())
    }

    fn undo(&mut self, scene: &mut crate::Scene) -> Result<()> {
        for shape_selection in self.selection.iter() {
            let shape = scene.shape_select_mut(shape_selection.0).expect("not 404");
            for coord_id in &shape_selection.1 {
                let (coord_id, coord) = {
                    let coord = shape.coord_select_mut(*coord_id).expect("not 404");
                    let movement = self.start_pos - self.end_pos;
                    let res_vec2 = coord.coord() + movement;
                    (coord.id, Coord::new(res_vec2.x, res_vec2.y))
                };

                shape.coord_set(coord_id, coord);
            }
        }
        Ok(())
    }

    fn merge(&self, futur: &dyn Command) -> Option<Result<Box<dyn Command>>> {
        if let Some(other) = futur.as_any().downcast_ref::<MoveCoords>() {
            if self.selection == other.selection && self.start_pos == other.start_pos {
                let new_end_pos = self.end_pos + other.end_pos - other.start_pos;
                let new_self = MoveCoords::new(self.selection.clone(), self.start_pos, new_end_pos);
                return Some(Ok(Box::new(new_self)));
            }
        }
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

    use super::MoveCoords;

    #[test]
    fn given_square_when_move_coords() {
        let mut scene = Scene::new();
        let shape = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.5, 0.5));
        let shape_id = scene.shape_insert(shape);
        let shape = scene.shape_select(shape_id).unwrap();
        let coord_id = shape.path[0].id;
        let selection = vec![(shape_id, vec![coord_id])];
        let start_pos = shape.path[0].coord();
        let end_pos = shape.path[0].coord() + Coord::new(0.5, 0.5);

        let mut command = MoveCoords::new(selection, start_pos, end_pos);
        command.execute(&mut scene).unwrap();

        let new_shape = scene.shape_select(shape_id).unwrap();
        let new_coord = new_shape.path[0].coord();
        assert_eq!(new_coord, end_pos);

        command.undo(&mut scene).unwrap();

        let new_shape = scene.shape_select(shape_id).unwrap();
        let new_coord = new_shape.path[0].coord();
        assert_eq!(new_coord, start_pos);
    }
}
