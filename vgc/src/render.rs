use crate::Vgc;
use common::pures::Vec2;
use common::Rgba;
use common::{pures::Affine, types::Coord};
pub trait VgcRenderer {
    fn create(&mut self) -> Result<(), String>;

    fn fill_background(&mut self, color: &Rgba) -> Result<(), String>;

    fn get_transform(&self) -> Result<Affine, String>;

    fn set_fill(&mut self, color: &Rgba) -> Result<(), String>;

    fn start_shape(&mut self, start_point: &Coord) -> Result<(), String>;

    fn move_curve(&mut self, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Result<(), String>;

    fn close_shape(&mut self) -> Result<(), String>;

    fn end(&mut self) -> Result<(), String>;
}

pub fn render_true<T>(canvas: &Vgc, renderer: &mut T) -> Result<(), String>
where
    T: VgcRenderer,
{
    renderer.create()?;
    renderer.fill_background(&canvas.background)?;
    let transform = renderer.get_transform()?;
    let m = &transform;

    for i_region in 0..canvas.shapes.len() {
        let region = &canvas.shapes[i_region];

        renderer.set_fill(&region.color)?;

        renderer.start_shape(&region.start.borrow().transform(m))?;

        for i_curve in 0..region.curves.len() {
            renderer.move_curve(
                &region.curves[i_curve].cp0.borrow().transform(m),
                &region.curves[i_curve].cp1.borrow().transform(m),
                &region.curves[i_curve].p1.borrow().transform(m),
            )?;
        }
        renderer.close_shape()?;
    }

    renderer.end()?;

    Ok(())
}

#[cfg(feature = "tiny-skia_renderer")]
use tiny_skia::{Paint, PathBuilder, Pixmap};

#[derive(Default)]
#[cfg(feature = "tiny-skia_renderer")]
pub struct TinySkiaRenderer<'a> {
    transform: Affine,
    pixmap: Option<Pixmap>,
    paint: Option<Paint<'a>>,
    current_path: Option<PathBuilder>,
}

impl<'a> TinySkiaRenderer<'a> {
    pub fn new(width: f32, height: f32) -> Self {
        let mut rtn = Self::default();
        rtn.transform = Affine::from_translate(Vec2::new(1.0, 1.0))
            .scale(Vec2::new(0.5, 0.5))
            .scale(Vec2::new(width, height));
        rtn
    }

    pub fn get_rgba(self) -> Option<Vec<u8>> {
        match self.pixmap {
            Some(pixmap) => Some(pixmap.take()),
            None => None,
        }
    }
}

#[cfg(feature = "tiny-skia_renderer")]
impl<'a> VgcRenderer for TinySkiaRenderer<'a> {
    fn create(&mut self) -> Result<(), String> {
        self.pixmap = Some(
            Pixmap::new(
                (self.transform.get_scale().x * 2.0) as u32,
                (self.transform.get_scale().y * 2.0) as u32,
            )
            .expect("Valid Size"),
        );
        Ok(())
    }

    fn fill_background(&mut self, color: &Rgba) -> Result<(), String> {
        let pixmap = self.pixmap.as_mut().expect("Valid Pixmap");
        pixmap.fill(tiny_skia::Color::from_rgba8(
            color.r, color.g, color.b, color.a,
        ));
        Ok(())
    }

    fn get_transform(&self) -> Result<Affine, String> {
        Ok(self.transform)
    }

    fn set_fill(&mut self, color: &Rgba) -> Result<(), String> {
        let mut paint = Paint::default();
        paint.set_color_rgba8(color.r, color.g, color.b, color.a);
        paint.anti_alias = true;
        self.paint = Some(paint);
        Ok(())
    }

    fn start_shape(&mut self, start_point: &Coord) -> Result<(), String> {
        let mut pb = PathBuilder::new();
        pb.move_to(start_point.x(), start_point.y());
        self.current_path = Some(pb);
        Ok(())
    }

    fn move_curve(&mut self, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Result<(), String> {
        let pb = self.current_path.as_mut().expect("Valid PathBuilder");
        pb.cubic_to(cp0.x(), cp0.y(), cp1.x(), cp1.y(), p1.x(), p1.y());
        Ok(())
    }

    fn close_shape(&mut self) -> Result<(), String> {
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

    fn end(&mut self) -> Result<(), String> {
        Ok(())
    }
}

mod test {

    #[test]
    #[cfg(feature = "tiny-skia_renderer")]
    fn test_tiny_skia_renderer() {
        use super::*;
        use crate::generate_from_push;
        use common::types::Coord;

        let vgc = generate_from_push(vec![vec![
            Coord::new(0.43 * 2.0 - 1.0, 0.27 * 2.0 - 1.0),
            Coord::new(0.06577811 * 2.0 - 1.0, 0.2938202 * 2.0 - 1.0),
            Coord::new(0.0 * 2.0 - 1.0, 1.0),
            Coord::new(0.0 * 2.0 - 1.0, 1.0),
            Coord::new(0.0 * 2.0 - 1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(0.7942219 * 2.0 - 1.0, 0.24617982 * 2.0 - 1.0),
            Coord::new(0.43 * 2.0 - 1.0, 0.27 * 2.0 - 1.0),
        ]]);

        let renderer = &mut TinySkiaRenderer::new(512.0, 512.0);

        let res = vgc.render(renderer);
        assert!(res.is_ok());

        let pixmap = renderer.pixmap.take().expect("Valid Pixmap");

        let black = tiny_skia::ColorU8::from_rgba(0, 0, 0, 255);
        let white = tiny_skia::ColorU8::from_rgba(255, 255, 255, 255);

        assert!(pixmap.pixel(511, 511).is_some());
        assert!(pixmap.pixel(514, 514).is_none());

        assert!(pixmap.pixel(107, 190).unwrap().demultiply() == white);
        assert!(pixmap.pixel(118, 200).unwrap().demultiply() == black);
        assert!(pixmap.pixel(482, 484).unwrap().demultiply() == black);
        assert!(pixmap.pixel(511, 494).unwrap().demultiply() == white);

        assert!(pixmap.encode_png().is_ok());
    }
}
