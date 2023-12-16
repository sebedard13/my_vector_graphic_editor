use vgc::VgcRenderer;
use web_sys::CanvasRenderingContext2d;

pub struct CanvasContext2DRender<'a> {
    context: &'a CanvasRenderingContext2d,
    translate: (f64, f64),
    w: f64,
    h: f64,
}

impl<'a> CanvasContext2DRender<'a> {
    pub fn new(context: &'a CanvasRenderingContext2d, translate: (f64, f64)) -> Self {
        Self { context, translate, w: 0.0, h:0.0 }
    }
}


impl<'a> VgcRenderer for CanvasContext2DRender<'a>{
    fn create(&mut self, width: u32, height: u32) -> Result<(), String> {
        self.w = width as f64;
        self.h = height as f64; 
        Ok(())
    }

    fn fill_background(&mut self, color: &vgc::Rgba) -> Result<(), String> {
        self.context.set_fill_style(&color.to_css_string().into());
        self.context.fill_rect(self.translate.0, self.translate.1, self.w, self.h);
        Ok(())
    }

    fn set_fill(&mut self, color: &vgc::Rgba) -> Result<(), String> {
        self.context.set_fill_style(&color.to_css_string().into());
        Ok(())
    }

    fn start_shape(&mut self, start_point: &vgc::coord::Coord) -> Result<(), String> {
        self.context.begin_path();
        self.context.move_to(self.translate.0 + start_point.x as f64, self.translate.1 + start_point.y as f64);
        Ok(())
    }

    fn move_curve(&mut self, cp0: &vgc::coord::Coord, cp1: &vgc::coord::Coord, p1: &vgc::coord::Coord) -> Result<(), String> {
        self.context.bezier_curve_to(self.translate.0 + cp0.x as f64, self.translate.1 + cp0.y as f64,
            self.translate.0 + cp1.x as f64, self.translate.1 + cp1.y as f64,
            self.translate.0 + p1.x as f64, self.translate.1 + p1.y as f64);
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
}