use super::Command;
use crate::LayerId;
use anyhow::{Ok, Result};
use macros::boxed;
use std::any::Any;

#[derive(Clone, Debug)]
pub struct ChangeStrokeSize {
    shapes: Vec<LayerId>,
    size: f32,
    old_sizes: Option<Vec<f32>>,
}

impl ChangeStrokeSize {
    #[boxed]
    pub fn new(shape_index: Vec<LayerId>, size: f32) -> Self {
        Self {
            shapes: shape_index,
            size,
            old_sizes: None,
        }
    }
}

impl Command for ChangeStrokeSize {
    fn execute(&mut self, scene: &mut crate::Scene) -> Result<()> {
        if self.old_sizes.is_none() {
            let mut old_sizes = Vec::new();
            for shape in self.shapes.iter() {
                if let Some(shape) = scene.shape_select_mut(*shape) {
                    old_sizes.push(shape.stroke.size.clone());
                    shape.stroke.size = self.size.clone();
                }
            }
            self.old_sizes = Some(old_sizes);
        }

        for shape in self.shapes.iter() {
            if let Some(shape) = scene.shape_select_mut(*shape) {
                shape.stroke.size = self.size.clone();
            }
        }
        Ok(())
    }

    fn undo(&mut self, scene: &mut crate::Scene) -> Result<()> {
        if let Some(old_sizes) = self.old_sizes.take() {
            for (shape_index, old_size) in self.shapes.iter().zip(old_sizes) {
                if let Some(shape) = scene.shape_select_mut(*shape_index) {
                    shape.stroke.size = old_size;
                }
            }
            Ok(())
        } else {
            Err(anyhow::anyhow!("Undoing a command that was not executed"))
        }
    }

    fn merge(&self, other: &dyn Command) -> Option<Result<Box<dyn Command>>> {
        if let Some(other) = other.as_any().downcast_ref::<ChangeStrokeSize>() {
            if self.shapes == other.shapes {
                let mut command = self.clone();
                command.size = other.size.clone();
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

    use super::ChangeStrokeSize;

    #[test]
    fn given_shape_when_change_color() {
        let mut scene = Scene::new();
        let mut s = Shape::new();
        s.color = Rgba::black();
        let id = scene.shape_insert(s);
        let expected = scene.clone();

        let mut command_handler = CommandsHandler::from(scene);
        let change_color = ChangeStrokeSize::new(vec![id], 1.0);

        let res = command_handler.execute(Box::new(change_color));

        assert!(res.is_ok());
        assert_eq!(
            command_handler
                .scene()
                .shape_select(id)
                .unwrap()
                .stroke
                .size,
            1.0
        );

        let res = command_handler.undo();

        assert!(res.is_ok());
        assert_eq!(
            command_handler
                .scene()
                .shape_select(id)
                .unwrap()
                .stroke
                .size,
            0.0
        );
        assert_eq!(*command_handler.scene(), expected);

        let res = command_handler.redo();

        assert!(res.is_ok());
        assert_eq!(
            command_handler
                .scene()
                .shape_select(id)
                .unwrap()
                .stroke
                .size,
            1.0
        );

        let res = command_handler.undo();

        assert!(res.is_ok());
        assert_eq!(
            command_handler
                .scene()
                .shape_select(id)
                .unwrap()
                .stroke
                .size,
            0.0
        );
    }
}
