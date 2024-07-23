use common::{pures::Affine, types::Coord, Rgba};

use crate::Scene;

#[cfg(feature = "tiny-skia_renderer")]
mod tiny_skia;

pub trait DrawingContext {
    fn create(&mut self) -> Result<(), String>;

    fn fill_background(&mut self, color: &Rgba) -> Result<(), String>;

    fn get_transform(&self) -> Result<Affine, String>;

    fn set_fill(&mut self, color: &Rgba) -> Result<(), String>;

    fn set_stroke(&mut self, color: &Rgba, size: f64) -> Result<(), String>;

    fn start_shape(&mut self, start_point: &Coord) -> Result<(), String>;

    fn move_curve(&mut self, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Result<(), String>;

    fn close_shape(&mut self) -> Result<(), String>;

    fn end(&mut self) -> Result<(), String>;
}

impl Scene {
    pub fn render<T: DrawingContext>(&self, renderer: &mut T) -> Result<(), String> {
        renderer.create()?;
        renderer.fill_background(&self.background)?;

        for region in self.layers.iter().rev() {
            region.value.render(renderer)?;
        }

        Ok(())
    }
}

#[cfg(test)]
pub struct MockDrawingContext {}

#[cfg(test)]
impl DrawingContext for MockDrawingContext {
    fn create(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn fill_background(&mut self, _color: &Rgba) -> Result<(), String> {
        Ok(())
    }

    fn get_transform(&self) -> Result<Affine, String> {
        Ok(Affine::identity())
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
}

