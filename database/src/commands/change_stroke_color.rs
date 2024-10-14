use super::Command;
use crate::LayerId;
use anyhow::{Ok, Result};
use common::Rgba;
use macros::boxed;
use std::any::Any;

#[derive(Clone, Debug)]
pub struct ChangeStrokeColor {
    shapes: Vec<LayerId>,
    color: Rgba,
    old_colors: Option<Vec<Rgba>>,
}

impl ChangeStrokeColor {
    #[boxed]
    pub fn new(shape_index: Vec<LayerId>, color: Rgba) -> Self {
        Self {
            shapes: shape_index,
            color,
            old_colors: None,
        }
    }
}

impl Command for ChangeStrokeColor {
    fn execute(&mut self, scene: &mut crate::Scene) -> Result<()> {
        if self.old_colors.is_none() {
            let mut old_colors = Vec::new();
            for shape in self.shapes.iter() {
                if let Some(shape) = scene.shape_select_mut(*shape) {
                    old_colors.push(shape.stroke.color.clone());
                    shape.stroke.color = self.color.clone();
                }
            }
            self.old_colors = Some(old_colors);
        }

        for shape in self.shapes.iter() {
            if let Some(shape) = scene.shape_select_mut(*shape) {
                shape.stroke.color = self.color.clone();
            }
        }
        Ok(())
    }

    fn undo(&mut self, scene: &mut crate::Scene) -> Result<()> {
        if let Some(old_colors) = self.old_colors.take() {
            for (shape_index, old_color) in self.shapes.iter().zip(old_colors) {
                if let Some(shape) = scene.shape_select_mut(*shape_index) {
                    shape.stroke.color = old_color;
                }
            }
            Ok(())
        } else {
            Err(anyhow::anyhow!("Undoing a command that was not executed"))
        }
    }

    fn merge(&self, other: &dyn Command) -> Option<Result<Box<dyn Command>>> {
        if let Some(other) = other.as_any().downcast_ref::<ChangeStrokeColor>() {
            if self.shapes == other.shapes {
                let mut command = self.clone();
                command.color = other.color.clone();
                return Some(Ok(Box::new(command)));
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
    use common::Rgba;

    use crate::{commands::CommandsHandler, Scene, Shape};

    use super::ChangeStrokeColor;

    #[test]
    fn given_shape_when_change_color() {
        let mut scene = Scene::new();
        let mut s = Shape::new();
        s.color = Rgba::black();
        let id = scene.shape_insert(s);
        let expected = scene.clone();

        let mut command_handler = CommandsHandler::from(scene);
        let change_color = ChangeStrokeColor::new(vec![id], Rgba::white());

        let res = command_handler.execute(Box::new(change_color));

        assert!(res.is_ok());
        assert_eq!(
            command_handler.scene().shape_select(id).unwrap().stroke.color,
            Rgba::white()
        );

        let res = command_handler.undo();

        assert!(res.is_ok());
        assert_eq!(
            command_handler.scene().shape_select(id).unwrap().stroke.color,
            Rgba::black()
        );
        assert_eq!(*command_handler.scene(), expected);

        let res = command_handler.redo();

        assert!(res.is_ok());
        assert_eq!(
            command_handler.scene().shape_select(id).unwrap().stroke.color,
            Rgba::white()
        );

        let res = command_handler.undo();

        assert!(res.is_ok());
        assert_eq!(
            command_handler.scene().shape_select(id).unwrap().stroke.color,
            Rgba::black()
        );
    }
}
