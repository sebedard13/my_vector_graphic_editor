use log::warn;

use crate::{
    commands::{Difference, Intersection, Union},
    UserSelection,
};

use super::SceneUserContext;

impl SceneUserContext {
    pub fn union(&mut self, selected: &mut UserSelection) -> Result<(), String> {
        if selected.shapes.len() < 2 {
            warn!("Union requires more than 1 shape to be selected");
        }

        for i in 1..selected.shapes.len() {
            let command = Union::boxed(selected.shapes[0].shape_id, selected.shapes[i].shape_id);
            self.command_handler
                .execute(command)
                .map_err(|e| e.to_string())?;
        }

        let mut a_selected = selected.shapes.remove(0);
        a_selected.coords.clear();
        selected.shapes.clear();
        selected.shapes.push(a_selected);

        Ok(())
    }

    pub fn difference(&mut self, selected: &mut UserSelection) -> Result<(), String> {
        if selected.shapes.len() < 2 {
            warn!("Difference requires more than 1 shape to be selected");
        }

        for i in 1..selected.shapes.len() {
            let command =
                Difference::boxed(selected.shapes[0].shape_id, selected.shapes[i].shape_id);
            self.command_handler
                .execute(command)
                .map_err(|e| e.to_string())?;
        }

        let mut a_selected = selected.shapes.remove(0);
        a_selected.coords.clear();
        selected.shapes.clear();
        selected.shapes.push(a_selected);

        Ok(())
    }

    pub fn intersection(&mut self, selected: &mut UserSelection) -> Result<(), String> {
        if selected.shapes.len() < 2 {
            warn!("Intersection requires more than 1 shape to be selected");
        }

        for i in 1..selected.shapes.len() {
            let command =
                Intersection::boxed(selected.shapes[0].shape_id, selected.shapes[i].shape_id);
            self.command_handler
                .execute(command)
                .map_err(|e| e.to_string())?;
        }

        let mut a_selected = selected.shapes.remove(0);
        a_selected.coords.clear();
        selected.shapes.clear();
        selected.shapes.push(a_selected);

        Ok(())
    }
}
