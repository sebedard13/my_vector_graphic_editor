use common::{types::Rect, Rgba};
use id::LayerId;
use serde::{Deserialize, Serialize};

pub mod id;
pub mod render;
#[macro_use]
pub mod shape;
pub mod tree_view;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
enum LayerType {
    Shape(shape::Shape),
    Folder,
}

impl LayerType {
    pub fn render(&self, renderer: &mut dyn render::DrawingContext) -> Result<(), String> {
        match self {
            LayerType::Shape(shape) => shape.render(renderer),
            LayerType::Folder => Ok(()),
        }
    }

    pub fn type_string(&self) -> String {
        match self {
            LayerType::Shape(_) => "Shape".to_string(),
            LayerType::Folder => "Folder".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct Layer {
    pub id: LayerId,
    pub name: String,
    pub value: LayerType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Scene {
    pub background: Rgba,

    //Index 0 is the foreground
    layers: Vec<Layer>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            background: Rgba::new(255, 255, 255, 0),
            layers: Vec::new(),
        }
    }

    fn layer_select(&self, index: LayerId) -> Option<&LayerType> {
        let find_result = self.layers.iter().find(|l| l.id == index);
        if let Some(layer) = find_result {
            return Some(&layer.value);
        }

        None
    }

    fn layer_select_mut(&mut self, index: LayerId) -> Option<&mut LayerType> {
        let find_result = self.layers.iter_mut().find(|l| l.id == index);
        if let Some(layer) = find_result {
            return Some(&mut layer.value);
        }

        None
    }

    pub fn layer_position(&self, index: LayerId) -> Option<usize> {
        self.layers.iter().position(|l| l.id == index)
    }

    pub fn layer_delete(&mut self, index: LayerId) {
        self.layers.retain(|l| l.id != index);
    }

    pub fn layer_move_up(&mut self, index: LayerId) {
        let index = self.layers.iter().position(|l| l.id == index).unwrap();
        if index > 0 {
            self.layers.swap(index, index - 1);
        }
    }

    pub fn layer_move_top(&mut self, index: LayerId) {
        let index = self.layers.iter().position(|l| l.id == index).unwrap();
        if index > 0 {
            self.layers[0..(index + 1)].rotate_right(1);
        }
    }

    pub fn layer_move_down(&mut self, index: LayerId) {
        let index = self.layers.iter().position(|l| l.id == index).unwrap();
        if index < self.layers.len() - 1 {
            self.layers.swap(index, index + 1);
        }
    }

    pub fn layer_move_before(
        &mut self,
        id_to_move: LayerId,
        id_position: LayerId,
    ) -> Result<(), String> {
        let index_position = self
            .layers
            .iter()
            .position(|l| l.id == id_position)
            .ok_or("id_position not found")?;
        self.layer_move_at(id_to_move, index_position)?;
        Ok(())
    }

    pub fn layer_move_at(&mut self, id_to_move: LayerId, index: usize) -> Result<(), String> {
        let index_to_move = self
            .layers
            .iter()
            .position(|l| l.id == id_to_move)
            .ok_or("id_to_move not found")?;
        let layer = self.layers.remove(index_to_move);
        self.layers.insert(index, layer);
        Ok(())
    }

    pub fn max_rect(&self) -> Rect {
        Rect::new(-1.0, -1.0, 1.0, 1.0)
    }

    pub fn debug_string(&self) -> String {
        let mut result = String::new();
        for layer in &self.layers {
            match &layer.value {
                LayerType::Shape(shape) => {
                    let path = format!("{}\n", shape.path());
                    result.push_str(&path);
                }
                LayerType::Folder => {
                    result.push_str("Folder\n");
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use crate::{LayerId, Scene, Shape};

    #[test]
    pub fn given_3_layers_when_move_top_then_id_correct() {
        let mut scene = Scene::new();
        let id1 = scene.shape_insert(Shape::new());
        let id2 = scene.shape_insert(Shape::new());
        let id3 = scene.shape_insert(Shape::new());

        scene.layer_move_top(id3);

        let layer_ids: Vec<LayerId> = scene.layers.iter().map(|l| l.id).collect();
        assert_eq!(layer_ids, vec![id3, id1, id2]);

        scene.layer_move_top(id1);

        let layer_ids: Vec<LayerId> = scene.layers.iter().map(|l| l.id).collect();
        assert_eq!(layer_ids, vec![id1, id3, id2]);
    }
}
