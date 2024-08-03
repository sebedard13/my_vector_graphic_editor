use common::{
    pures::Affine,
    types::{Coord, ScreenRect},
    Rgba,
};
//use transparent_grid::render_transparent_grid;

use crate::{LayerId, Scene};

#[cfg(feature = "tiny-skia_renderer")]
mod tiny_skia;

mod transparent_grid;

pub trait DrawingContext {
    fn create(&mut self) -> Result<(), String>;

    fn fill_background(&mut self, color: &Rgba) -> Result<(), String>;

    fn get_transform(&self) -> Result<Affine, String>;

    fn get_max_view(&self) -> Result<ScreenRect, String>;

    fn set_fill(&mut self, color: &Rgba) -> Result<(), String>;

    fn set_stroke(&mut self, color: &Rgba, size: f64) -> Result<(), String>;

    fn start_shape(&mut self, start_point: &Coord) -> Result<(), String>;

    fn move_curve(&mut self, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Result<(), String>;

    fn move_line(&mut self, p: &Coord) -> Result<(), String>;

    fn close_shape(&mut self) -> Result<(), String>;

    fn end(&mut self) -> Result<(), String>;
}

#[derive(Debug, Default)]
pub struct RenderOption {
    /// If set, only render the layers up to this layer not included
    pub to_layer: Option<LayerId>,
    /// If set, skip the layers with these ids
    pub skip_layers: Vec<LayerId>,
    /// If set, only render the layers with these ids
    pub only_layers: Vec<LayerId>,
}

impl Scene {
    pub fn render<T: DrawingContext>(&self, renderer: &mut T) -> Result<(), String> {
        self.render_with_options(renderer, RenderOption::default())
    }

    pub fn render_with_options(
        &self,
        renderer: &mut impl DrawingContext,
        options: RenderOption,
    ) -> Result<(), String> {
        renderer.create()?;
        //render_transparent_grid(renderer)?;
        renderer.fill_background(&self.background)?;

        for layer in self.layers.iter().rev() {
            if let Some(to_layer) = options.to_layer {
                if layer.id == to_layer {
                    break;
                }
            }

            if options.skip_layers.contains(&layer.id) {
                continue;
            }

            if options.only_layers.len() == 0 || options.only_layers.contains(&layer.id) {
                layer.value.render(renderer)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
pub struct MockDrawingContext {
    pub transform: Affine,
    pub max_view: ScreenRect,
}

#[cfg(test)]
impl Default for MockDrawingContext {
    fn default() -> Self {
        use common::types::{ScreenCoord, ScreenRect};
        Self {
            transform: Affine::identity(),
            max_view: ScreenRect {
                top_left: ScreenCoord::new(-1.0, 1.0),
                bottom_right: ScreenCoord::new(1.0, -1.0),
            },
        }
    }
}

#[cfg(test)]
impl DrawingContext for MockDrawingContext {
    fn create(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn fill_background(&mut self, _color: &Rgba) -> Result<(), String> {
        Ok(())
    }

    fn get_transform(&self) -> Result<Affine, String> {
        Ok(self.transform)
    }

    fn get_max_view(&self) -> Result<ScreenRect, String> {
        Ok(self.max_view)
    }

    fn set_fill(&mut self, _color: &Rgba) -> Result<(), String> {
        Ok(())
    }

    fn set_stroke(&mut self, _color: &Rgba, _size: f64) -> Result<(), String> {
        Ok(())
    }

    fn start_shape(&mut self, _start_point: &Coord) -> Result<(), String> {
        Ok(())
    }

    fn move_curve(&mut self, _cp0: &Coord, _cp1: &Coord, _p1: &Coord) -> Result<(), String> {
        Ok(())
    }

    fn close_shape(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn end(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn move_line(&mut self, _: &Coord) -> Result<(), String> {
        Ok(())
    }
}
