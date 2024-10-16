use crate::UserSelection;

use super::SceneUserContext;

impl SceneUserContext {
    pub fn union(&self, selected: &mut UserSelection) -> Result<(), String> {
        if selected.shapes.len() == 2 {
            log::info!("Union selected");
        } else {
            log::warn!("Union requires exactly 2 shapes to be selected");
        }
        Ok(())
    }

    pub fn difference(&self, selected: &mut UserSelection) -> Result<(), String> {
        if selected.shapes.len() == 2 {
            log::info!("Difference selected");
        } else {
            log::warn!("Difference requires exactly 2 shapes to be selected");
        }
        Ok(())
    }

    pub fn intersection(&self, selected: &mut UserSelection) -> Result<(), String> {
        if selected.shapes.len() == 2 {
            log::info!("Intersection selected");
        } else {
            log::warn!("Intersection requires exactly 2 shapes to be selected");
        }
        Ok(())
    }
}
