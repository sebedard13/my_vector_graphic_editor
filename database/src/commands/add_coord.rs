use super::Command;
use crate::{CoordId, DbCoord, LayerId};
use anyhow::{Context, Error, Ok, Result};
use macros::boxed;
use std::any::Any;

#[derive(Clone, Debug)]
pub struct AddCoord {
    shapes: LayerId,
    curve: usize,
    t: f32,
    coord_to_undo: Option<CoordId>,
    cp_to_undo: Option<(DbCoord, DbCoord)>,
}

impl AddCoord {
    #[boxed]
    pub fn new(shape_index: LayerId, curve: usize, t: f32) -> Self {
        Self {
            shapes: shape_index,
            curve,
            t,
            coord_to_undo: None,
            cp_to_undo: None,
        }
    }
}

impl Command for AddCoord {
    fn execute(&mut self, scene: &mut crate::Scene) -> Result<()> {
        let shape = scene
            .shape_select_mut(self.shapes)
            .context("Shape not found")?;
        let changed_curve = shape.curve_select(self.curve).context("Curve not found")?;
        self.cp_to_undo = Some((changed_curve.cp0.clone(), changed_curve.cp1.clone()));

        let coord_ids = shape.curve_insert_smooth(self.curve, self.t);
        self.coord_to_undo = Some(coord_ids.1);
        Ok(())
    }

    fn undo(&mut self, scene: &mut crate::Scene) -> Result<()> {
        let shape = scene
            .shape_select_mut(self.shapes)
            .context("Shape not found")?;
        shape
            .coord_delete(self.coord_to_undo.take().context("No coord to undo")?)
            .map_err(Error::msg)?;

        if let Some((cp0, cp1)) = self.cp_to_undo.take() {
            shape.coord_set(cp0.id, cp0.coord());
            shape.coord_set(cp1.id, cp1.coord());
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

    use common::{
        pures::Affine,
        types::{Coord, Length2d},
    };

    use crate::{commands::Command, DbCoord, Scene, Shape};

    use super::AddCoord;

    #[test]
    fn given_square_when_add_coord_in_path() {
        let mut scene = Scene::new();
        let shape = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.5, 0.5));
        let expected = shape.path.clone();
        let shape_id = scene.shape_insert(shape);

        let mut command = AddCoord::new(shape_id, 0, 0.3);
        command.execute(&mut scene).unwrap();

        let new_shape = scene.shape_select(shape_id).unwrap();

        assert_eq!(new_shape.curves_len(), 5);

        command.undo(&mut scene).unwrap();

        let new_shape = scene.shape_select(shape_id).unwrap();
        assert_eq!(expected, new_shape.path);
    }

    #[test]
    fn given_square_when_add_coord() {
        let mut scene = Scene::new();
        let shape = Shape::new_from_lines(
            vec![
                DbCoord::new(0.0, 0.0),
                DbCoord::new(1.0, 0.0),
                DbCoord::new(1.0, 1.0),
                DbCoord::new(0.0, 1.0),
            ],
            Affine::identity(),
        );
        let expected = shape.path.clone();
        let shape_id = scene.shape_insert(shape);

        let mut command = AddCoord::new(shape_id, 0, 0.3);
        command.execute(&mut scene).unwrap();

        let new_shape = scene.shape_select(shape_id).unwrap();

        assert_eq!(new_shape.curves_len(), 5);

        command.undo(&mut scene).unwrap();

        let new_shape = scene.shape_select(shape_id).unwrap();
        assert_eq!(expected, new_shape.path);
    }
}
