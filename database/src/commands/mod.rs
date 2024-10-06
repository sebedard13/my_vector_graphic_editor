use std::{any::Any, fmt::Debug};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::Scene;
mod add_coord;
mod change_color;
mod move_coords;
mod move_layer;
mod remove_coord;
mod toggle_handle;

pub use add_coord::AddCoord;
pub use change_color::ChangeColor;
pub use move_coords::MoveCoords;
pub use move_layer::MoveLayer;
pub use remove_coord::RemoveCoord;
pub use toggle_handle::ToggleHandle;

pub trait Command: Any + Debug {
    fn execute(&mut self, scene: &mut Scene) -> Result<()>;
    fn undo(&mut self, scene: &mut Scene) -> Result<()>;

    /// Merge two commands if possible to save memory
    /// Returns None if the commands can't be merged
    fn merge(&self, futur: &dyn Command) -> Option<Result<Box<dyn Command>>>;

    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandsHandler {
    scene: Scene,
    #[serde(skip)]
    stack: Vec<Box<dyn Command>>,
    #[serde(skip)]
    index: usize,
}

impl From<Scene> for CommandsHandler {
    fn from(scene: Scene) -> Self {
        Self {
            scene,
            stack: Vec::new(),
            index: 0,
        }
    }
}

impl CommandsHandler {
    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn unsafe_scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn execute(&mut self, mut command: Box<dyn Command>) -> Result<()> {
        command.execute(&mut self.scene)?;
        self.stack.truncate(self.index);
        if let Some(&prev_command) = self.stack.last().as_ref() {
            if let Some(merged) = prev_command.merge(command.as_ref()) {
                let merged_command = merged?;

                self.stack.pop();
                self.stack.push(merged_command);

                return Ok(());
            }
        }
        self.stack.push(command);
        self.index += 1;
        Ok(())
    }

    pub fn undo(&mut self) -> Result<()> {
        if self.index == 0 {
            return Ok(());
        }

        self.stack[self.index - 1].undo(&mut self.scene)?;
        self.index -= 1;
        Ok(())
    }

    pub fn redo(&mut self) -> Result<()> {
        if self.index == self.stack.len() {
            return Ok(());
        }

        self.stack[self.index].execute(&mut self.scene)?;
        self.index += 1;
        Ok(())
    }
}
