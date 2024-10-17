use crate::{
    commands::{Difference, Intersection, Union},
    UserSelection,
};

use super::SceneUserContext;

impl SceneUserContext {
    pub fn union(&mut self, selected: &mut UserSelection) -> Result<(), String> {
        if selected.shapes.len() == 2 {
            let command = Union::boxed(selected.shapes[0].shape_id, selected.shapes[1].shape_id);
            self.command_handler
                .execute(command)
                .map_err(|e| e.to_string())?;
        } else {
            log::warn!("Union requires exactly 2 shapes to be selected");
        }
        Ok(())
    }

    pub fn difference(&mut self, selected: &mut UserSelection) -> Result<(), String> {
        if selected.shapes.len() == 2 {
            let command =
                Difference::boxed(selected.shapes[0].shape_id, selected.shapes[1].shape_id);
            self.command_handler
                .execute(command)
                .map_err(|e| e.to_string())?;
        } else {
            log::warn!("Difference requires exactly 2 shapes to be selected");
        }
        Ok(())
    }

    pub fn intersection(&mut self, selected: &mut UserSelection) -> Result<(), String> {
        if selected.shapes.len() == 2 {
            let command =
                Intersection::boxed(selected.shapes[0].shape_id, selected.shapes[1].shape_id);
            self.command_handler
                .execute(command)
                .map_err(|e| e.to_string())?;
        } else {
            log::warn!("Intersection requires exactly 2 shapes to be selected");
        }
        Ok(())
    }
}
