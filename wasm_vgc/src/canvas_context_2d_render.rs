use vgc::VgcRenderer;
use web_sys::CanvasRenderingContext2d;
use common::types::Coord;
use common::Rgba;

pub struct CanvasContext2DRender<'a> {
    context: &'a CanvasRenderingContext2d,
    translate: (f64, f64),
    w: f64,
    h: f64,
}

impl<'a> CanvasContext2DRender<'a> {
    pub fn new(
        context: &'a CanvasRenderingContext2d,
        translate: (f64, f64),
        w: f64,
        h: f64,
    ) -> Self {
        Self {
            context,
            translate,
            w,
            h,
        }
    }
}

impl<'a> VgcRenderer for CanvasContext2DRender<'a> {
    fn create(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn fill_background(
        &mut self,
        color: &Rgba,
        max_coord: &Coord,
    ) -> Result<(), String> {
        self.context.set_fill_style(&color.to_css_string().into());
        self.context.fill_rect(
            self.translate.0,
            self.translate.1,
            self.w * max_coord.x() as f64,
            self.h * max_coord.y() as f64,
        );
        Ok(())
    }

    fn set_fill(&mut self, color: &Rgba) -> Result<(), String> {
        self.context.set_fill_style(&color.to_css_string().into());
        Ok(())
    }

    fn start_shape(&mut self, start_point: &Coord) -> Result<(), String> {
        self.context.begin_path();
        self.context
            .move_to(start_point.x() as f64, start_point.y() as f64);
        Ok(())
    }

    fn move_curve(
        &mut self,
        cp0: &Coord,
        cp1: &Coord,
        p1: &Coord,
    ) -> Result<(), String> {
        self.context.bezier_curve_to(
            cp0.x() as f64,
            cp0.y() as f64,
            cp1.x() as f64,
            cp1.y() as f64,
            p1.x() as f64,
            p1.y() as f64,
        );
        Ok(())
    }

    fn close_shape(&mut self) -> Result<(), String> {
        self.context.close_path();
        self.context.fill();
        Ok(())
    }

    fn end(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn get_transform(&self) -> Result<(f32, f32, f32, f32), String> {
        Ok((
            self.translate.0 as f32,
            self.translate.1 as f32,
            self.w as f32,
            self.h as f32,
        ))
    }
}
