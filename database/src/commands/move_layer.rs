#![allow(dead_code)]
use super::Command;
use crate::LayerId;
use anyhow::{Context, Error, Ok, Result};
use std::any::Any;

#[derive(Clone, Debug)]
pub struct MoveLayer {
    layer: LayerId,
    move_to: LayerId,
    undo_pos: Option<usize>,
}

impl MoveLayer {
    pub fn new(layer: LayerId, move_to: LayerId) -> Self {
        Self {
            layer,
            move_to,
            undo_pos: None,
        }
    }

    pub fn boxed(layer: LayerId, move_to: LayerId) -> Box<Self> {
        Box::new(Self::new(layer, move_to))
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
mod test {}
