use std::any::Any;

use common::{types::Rect, Rgba};
use id::LayerId;

pub mod id;
pub mod render;
#[macro_use]
pub mod shape;

enum LayerType {
    Shape,
    Folder,
}

pub trait LayerValue: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn render(&self, renderer: &mut dyn render::DrawingContext) -> Result<(), String>;
}

///Macro to implement the LayerValue trait  for a type.
/// It implement as_any and as_any_mut and leave room for other functions.
#[macro_export]
macro_rules! impl_layer_value {
    ($type:ty, $($functions:item)*) => {
        impl LayerValue for $type {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }

            $($functions)*
        }
    };
}

struct Layer {
    pub id: LayerId,
    layer_type: LayerType,
    pub value: Box<dyn LayerValue>,
}

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

    fn layer_select<T: 'static>(&self, index: LayerId) -> Option<&T> {
        let find_result = self.layers.iter().find(|l| l.id == index);
        if let Some(layer) = find_result {
            return layer.value.as_any().downcast_ref::<T>();
        }

        None
    }

    fn layer_select_mut<T: 'static>(&mut self, index: LayerId) -> Option<&mut T> {
        let find_result = self.layers.iter_mut().find(|l| l.id == index);
        if let Some(layer) = find_result {
            return layer.value.as_any_mut().downcast_mut();
        }

        None
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

    pub fn layer_move_at(&mut self, id_to_move: LayerId, id_position: LayerId) {
        let index_to_move = self.layers.iter().position(|l| l.id == id_to_move).unwrap();
        let index_position = self
            .layers
            .iter()
            .position(|l| l.id == id_position)
            .unwrap();
        let layer = self.layers.remove(index_to_move);
        self.layers.insert(index_position, layer);
    }

    pub fn max_rect(&self) -> Rect {
        Rect::new(-1.0, -1.0, 1.0, 1.0)
    }

    pub fn debug_string(&self) -> String {
        //TODO
        String::new()
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
