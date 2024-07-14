use common::{pures::Affine, types::Coord, Rgba};

use crate::Scene;

pub trait DrawingContext {
    fn create(&mut self) -> Result<(), String>;

    fn fill_background(&mut self, color: &Rgba) -> Result<(), String>;

    fn get_transform(&self) -> Result<Affine, String>;

    fn set_fill(&mut self, color: &Rgba) -> Result<(), String>;

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
