use common::pures::Affine;
use common::types::Coord;
use common::Rgba;
use vgc::VgcRenderer;
use web_sys::CanvasRenderingContext2d;

pub struct CanvasContext2DRender<'a> {
    context: &'a CanvasRenderingContext2d,
    transform: Affine,
}

impl<'a> CanvasContext2DRender<'a> {
    pub fn new(context: &'a CanvasRenderingContext2d, transform: Affine) -> Self {
        Self { context, transform }
    }
}

impl<'a> VgcRenderer for CanvasContext2DRender<'a> {
    fn create(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn fill_background(&mut self, color: &Rgba) -> Result<(), String> {
        self.context.set_fill_style(&color.to_css_string().into());

        let corner0 = Coord::new(-1.0, -1.0).transform(&self.transform);
        let corner1 = Coord::new(-1.0, 1.0).transform(&self.transform);
        let corner2 = Coord::new(1.0, 1.0).transform(&self.transform);
        let corner3 = Coord::new(1.0, -1.0).transform(&self.transform);

        self.context.begin_path();
        self.context.move_to(corner0.x() as f64, corner0.y() as f64);
        self.context.line_to(corner1.x() as f64, corner1.y() as f64);
        self.context.line_to(corner2.x() as f64, corner2.y() as f64);
        self.context.line_to(corner3.x() as f64, corner3.y() as f64);
        self.context.close_path();
        self.context.fill();

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

    fn move_curve(&mut self, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Result<(), String> {
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

    fn get_transform(&self) -> Result<Affine, String> {
        Ok(self.transform)
    }
}
