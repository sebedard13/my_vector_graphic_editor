use common::types::Coord;
use common::Rgba;
use common::{pures::Affine, types::ScreenRect};
use database::DrawingContext;
use web_sys::{CanvasRenderingContext2d, CanvasWindingRule};

pub struct CanvasContext2DRender<'a> {
    context: &'a CanvasRenderingContext2d,
    transform: Affine,
    max_view: ScreenRect,
}

impl<'a> CanvasContext2DRender<'a> {
    pub fn new(
        context: &'a CanvasRenderingContext2d,
        transform: Affine,
        max_view: ScreenRect,
    ) -> Self {
        Self {
            context,
            transform,
            max_view,
        }
    }
}

impl<'a> DrawingContext for CanvasContext2DRender<'a> {
    fn create(&mut self) -> Result<(), String> {
        self.context
            .set_fill_style(&String::from("rgba(0, 0, 0, 0)").into());
        self.context
            .set_stroke_style(&String::from("rgba(0, 0, 0, 0)").into());
        self.context.set_line_width(0.0);
        Ok(())
    }

    fn get_transform(&self) -> Result<Affine, String> {
        Ok(self.transform)
    }

    fn get_max_view(&self) -> Result<ScreenRect, String> {
        Ok(self.max_view)
    }

    fn fill_background(&mut self, color: &Rgba) -> Result<(), String> {
        self.context.set_fill_style(&color.to_css_string().into());

        let corner0 = self.transform * Coord::new(-1.0, -1.0);
        let corner1 = self.transform * Coord::new(-1.0, 1.0);
        let corner2 = self.transform * Coord::new(1.0, 1.0);
        let corner3 = self.transform * Coord::new(1.0, -1.0);

        self.context.begin_path();
        self.context.move_to(corner0.x as f64, corner0.y as f64);
        self.context.line_to(corner1.x as f64, corner1.y as f64);
        self.context.line_to(corner2.x as f64, corner2.y as f64);
        self.context.line_to(corner3.x as f64, corner3.y as f64);
        self.context.close_path();
        self.context.fill();

        Ok(())
    }

    fn set_fill(&mut self, color: &Rgba) -> Result<(), String> {
        self.context.set_fill_style(&color.to_css_string().into());
        Ok(())
    }

    fn set_stroke(&mut self, color: &Rgba, size: f64) -> Result<(), String> {
        self.context.set_stroke_style(&color.to_css_string().into());
        self.context.set_line_width(size);
        Ok(())
    }

    fn start_shape(&mut self, start_point: &Coord) -> Result<(), String> {
        self.context.begin_path();
        self.context
            .move_to(start_point.x as f64, start_point.y as f64);
        Ok(())
    }

    fn move_curve(&mut self, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Result<(), String> {
        self.context.bezier_curve_to(
            cp0.x as f64,
            cp0.y as f64,
            cp1.x as f64,
            cp1.y as f64,
            p1.x as f64,
            p1.y as f64,
        );
        Ok(())
    }

    fn move_line(&mut self, p: &Coord) -> Result<(), String> {
        self.context.line_to(p.x as f64, p.y as f64);
        Ok(())
    }

    fn close_shape(&mut self) -> Result<(), String> {
        self.context.close_path();
        self.context
            .fill_with_canvas_winding_rule(CanvasWindingRule::Evenodd);
        self.context.stroke();
        Ok(())
    }

    fn end(&mut self) -> Result<(), String> {
        Ok(())
    }
}
