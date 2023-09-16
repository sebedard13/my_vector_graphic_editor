use crate::{Vgc, fill::Rgba, coord::Coord};



pub trait VgcRenderer{
    fn create(&mut self, width: u32, height: u32) -> Result<(), String>;

    fn fill_background(&mut self, color: &Rgba) -> Result<(), String>;

    fn set_fill(&mut self, color: &Rgba) -> Result<(), String>;

    fn start_shape(&mut self, start_point: &Coord) -> Result<(), String>;

    fn move_curve(&mut self, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Result<(), String>;

    fn close_shape(&mut self) -> Result<(), String>;

    fn end(&mut self) -> Result<(), String>;
}

//TODO scale or not to scale unit
pub fn render_true<T>(canvas:&Vgc, renderer: &mut T, w: u32, h: u32) -> Result<(), String>
where T : VgcRenderer {

    renderer.create(w, h)?;
    renderer.fill_background(&canvas.background)?;

    for i_region in 0..canvas.shapes.len() {
        let region = &canvas.shapes[i_region];
    
        renderer.set_fill(&region.color)?;

        renderer.start_shape(&region.start.borrow().scale(w, h))?;

        for i_curve in 0..region.curves.len() {
            renderer.move_curve(
                &region.curves[i_curve].cp0.borrow().scale(w, h),
                &region.curves[i_curve].cp1.borrow().scale(w, h),
                &region.curves[i_curve].p1.borrow().scale(w, h),
            )?;
        }
        renderer.close_shape()?;
    }

    renderer.end()?;

    Ok(())
}

#[cfg(feature = "tiny-skia_renderer")]
use tiny_skia::{Pixmap, Paint, PathBuilder};

#[derive(Default)]
#[cfg(feature = "tiny-skia_renderer")]
pub struct TinySkiaRenderer<'a> {
    pixmap: Option<Pixmap>,
    paint: Option<Paint<'a>>,
    current_path: Option<PathBuilder>,
}

impl<'a> TinySkiaRenderer<'a>{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn get_rgba(self) -> Option<Vec<u8>>{
        match self.pixmap{
            Some(pixmap) => Some(pixmap.take()),
            None => None,
        }
    }
}

#[cfg(feature = "tiny-skia_renderer")]
impl<'a> VgcRenderer for TinySkiaRenderer<'a>{
    fn create(&mut self, width: u32, height: u32) -> Result<(), String>{
        self.pixmap = Some(Pixmap::new(width, height).expect("Valid Size"));
        Ok(())
    }

    fn fill_background(&mut self, color: &Rgba) -> Result<(), String> {
        let pixmap = self.pixmap.as_mut().expect("Valid Pixmap");
        pixmap.fill(tiny_skia::Color::from_rgba8(
            color.r,
            color.g,
            color.b,
            color.a,
        ));
        Ok(())
    }

    fn set_fill(&mut self, color: &Rgba) -> Result<(), String>{
        let mut paint = Paint::default();
        paint.set_color_rgba8(
            color.r,
            color.g,
            color.b,
            color.a,
        );
        paint.anti_alias = true;
        self.paint = Some(paint);
        Ok(())
    }

    fn start_shape(&mut self, start_point: &Coord) -> Result<(), String>{
        let mut pb = PathBuilder::new();
        pb.move_to(start_point.x, start_point.y);
        self.current_path = Some(pb);
        Ok(())
    }

    fn move_curve(&mut self, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Result<(), String>{
        let pb = self.current_path.as_mut().expect("Valid PathBuilder");
        pb.cubic_to(cp0.x, cp0.y, cp1.x, cp1.y, p1.x, p1.y);
        Ok(())
    }

    fn close_shape(&mut self) -> Result<(), String>{
        let pb = self.current_path.take().expect("Valid PathBuilder");
        let path = pb.finish().expect("Valid Path");

        let pixmap = self.pixmap.as_mut().expect("Valid Pixmap");
        let paint = self.paint.as_ref().expect("Valid Paint");

        pixmap.fill_path(
            &path,
            &paint,
            tiny_skia::FillRule::Winding,
            tiny_skia::Transform::identity(),
            None,
        );
        Ok(())
    }

    fn end(&mut self) -> Result<(), String>{
        Ok(())
    }
}

mod test {

    #[test]
    #[cfg(feature = "tiny-skia_renderer")]
    fn test_tiny_skia_renderer(){
        use std::fs;
        use super::*;
        use crate::coord::Coord;
        use crate::generate_from_push;

        let vgc = generate_from_push(vec![vec![
            Coord::new(0.43, 0.27),
            Coord::new(0.06577811, 0.2938202),
            Coord::new(0.0, 1.0),
            Coord::new(0.0, 1.0),
            Coord::new(0.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(0.7942219, 0.24617982),
            Coord::new(0.43, 0.27),
        ]]);

        let renderer = &mut TinySkiaRenderer::default();

        let res = vgc.render_w(renderer, 500);
        assert!(res.is_ok());

        let pixmap = renderer.pixmap.take().expect("Valid Pixmap");
        let image = pixmap.encode_png().expect("Valid Image");

        fs::write("test.png", image).expect("Unable to write file");
    }
}