use super::Command;
use crate::{CoordId, DbCoord, LayerId};
use anyhow::{Context, Ok, Result};
use macros::boxed;
use std::any::Any;

#[derive(Clone, Debug)]
pub struct ToggleHandle {
    shapes: LayerId,
    coord: CoordId,
    cp_to_undo: Option<(DbCoord, DbCoord)>,
}

impl ToggleHandle {
    #[boxed]
    pub fn new(shape_index: LayerId, curve: CoordId) -> Self {
        Self {
            shapes: shape_index,
            coord: curve,
            cp_to_undo: None,
        }
    }
}

impl Command for ToggleHandle {
    fn execute(&mut self, scene: &mut crate::Scene) -> Result<()> {
        let shape = scene
            .shape_select_mut(self.shapes)
            .context("Shape not found")?;
        let curve_index = shape
            .curve_select_of_coord_id(self.coord)
            .context("Curve not found")?;
        let curve = shape.curve_select(curve_index).context("Curve not found")?;
        let curve_after = shape
            .curve_select((curve_index + 1) % shape.curves_len())
            .context("Curve not found")?;
        self.cp_to_undo = Some((curve.cp1.clone(), curve_after.cp0.clone()));

        shape.toggle_separate_join_handle(curve_index);

        Ok(())
    }

    fn undo(&mut self, scene: &mut crate::Scene) -> Result<()> {
        let shape = scene
            .shape_select_mut(self.shapes)
            .context("Shape not found")?;

        let curve_index = shape
            .curve_select_of_coord_id(self.coord)
            .context("Curve not found")?;
        let path_len = shape.path.len();
        let curves_len = shape.curves_len();
        shape.path[(curve_index * 3 + 2) % path_len] =
            self.cp_to_undo.as_ref().context("No coord to undo")?.0;
        shape.path[((curve_index + 1) % curves_len * 3 + 1) % path_len] =
            self.cp_to_undo.as_ref().context("No coord to undo")?.1;

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

    use super::ToggleHandle;

    #[test]
    fn given_circle_when_toggle_coord() {
        let mut scene = Scene::new();
        let shape = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.5, 0.5));
        let coord_id = shape.path[0].id;
        let expected = shape.path.clone();
        let shape_id = scene.shape_insert(shape);

        let mut command = ToggleHandle::new(shape_id, coord_id);
        command.execute(&mut scene).unwrap();

        let new_shape = scene.shape_select(shape_id).unwrap();

        assert_eq!(new_shape.path[1], new_shape.path[0]);

        command.undo(&mut scene).unwrap();

        let new_shape = scene.shape_select(shape_id).unwrap();
        assert_eq!(expected, new_shape.path);
    }

    #[test]
    fn given_square_when_toggle_coord() {
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
        let coord_id = shape.path[0].id;
        let shape_id = scene.shape_insert(shape);

        let mut command = ToggleHandle::new(shape_id, coord_id);
        command.execute(&mut scene).unwrap();

        let new_shape = scene.shape_select(shape_id).unwrap();

        assert_ne!(new_shape.path[1], new_shape.path[0]);

        command.undo(&mut scene).unwrap();

        let new_shape = scene.shape_select(shape_id).unwrap();
        assert_eq!(expected, new_shape.path);
    }
}
