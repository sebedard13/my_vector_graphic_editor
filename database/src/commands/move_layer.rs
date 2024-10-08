use super::Command;
use crate::LayerId;
use anyhow::{Context, Error, Ok, Result};
use macros::boxed;
use std::any::Any;

#[derive(Clone, Debug)]
pub struct MoveLayer {
    layer: LayerId,
    move_to: LayerId,
    undo_pos: Option<usize>,
}

impl MoveLayer {
    #[boxed]
    pub fn new(layer: LayerId, move_to: LayerId) -> Self {
        Self {
            layer,
            move_to,
            undo_pos: None,
        }
    }
}

impl Command for MoveLayer {
    fn execute(&mut self, scene: &mut crate::Scene) -> Result<()> {
        let before_pos = scene
            .layer_position(self.layer)
            .context("layer not found")?;
        scene
            .layer_position(self.move_to)
            .context("layer not found")?;
        scene
            .layer_move_before(self.layer, self.move_to)
            .map_err(Error::msg)?;

        self.undo_pos = Some(before_pos);

        Ok(())
    }

    fn undo(&mut self, scene: &mut crate::Scene) -> Result<()> {
        let before_pos = self.undo_pos.context("no position to undo")?;

        scene
            .layer_move_at(self.layer, before_pos)
            .map_err(Error::msg)?;
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
    use common::types::{Coord, Length2d};

    use crate::{commands::{Command, MoveLayer}, Scene, Shape};


    #[test]
    fn given_scene_2_layers_when_move_up(){
        let mut scene = Scene::new();
        let layer1 = scene.shape_insert(Shape::new_circle(Coord::new(0.0,0.0), Length2d::new(0.5,0.5)));
        let layer2 = scene.shape_insert(Shape::new_circle(Coord::new(0.0,0.0), Length2d::new(0.5,0.5)));
    
        let mut move_layer = MoveLayer::new(layer2, layer1);
        move_layer.execute(&mut scene).unwrap();
        assert_eq!(scene.layer_position(layer2).unwrap(), 0);
        assert_eq!(scene.layer_position(layer1).unwrap(), 1);

        move_layer.undo(&mut scene).unwrap();
        assert_eq!(scene.layer_position(layer2).unwrap(), 1);
        assert_eq!(scene.layer_position(layer1).unwrap(), 0);
    }

}
