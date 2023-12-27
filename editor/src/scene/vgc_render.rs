use iced::widget::canvas::path::Builder;
use iced::widget::canvas::Frame;
use iced::widget::canvas::{Fill, Path};
use iced::{Color, Point};
use vgc::coord::Coord;
use vgc::{Rgba, VgcRenderer};

pub struct IcedFrame<'a> {
    frame: &'a mut Frame,
    fill: Option<Fill>,
    path_builder: Option<Builder>,
    ratio: f64,
}

impl IcedFrame<'_> {
    pub fn new(frame: &mut Frame, ratio: f64) -> IcedFrame {
        IcedFrame {
            frame: frame,
            fill: None,
            path_builder: None,
            ratio,
        }
    }
}

impl VgcRenderer for IcedFrame<'_> {
    fn create(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn fill_background(&mut self, _: &Rgba) -> Result<(), String> {
        Ok(())
    }

    fn get_transform(&self) -> Result<(f32, f32, f32, f32), String> {
        Ok((0.0, 0.0, 1.0, 1.0 / self.ratio as f32))
    }

    fn set_fill(&mut self, color: &Rgba) -> Result<(), String> {
        self.fill = Some(Fill::from(Color::from_rgba8(
            color.r,
            color.g,
            color.b,
            color.a as f32 / 255.0,
        )));
        Ok(())
    }

    fn start_shape(&mut self, start_point: &Coord) -> Result<(), String> {
        let mut builder = Builder::new();
        builder.move_to(Point::new(start_point.x, start_point.y));
        self.path_builder = Some(builder);
        Ok(())
    }

    fn move_curve(
        &mut self,
        cp0: &vgc::coord::Coord,
        cp1: &vgc::coord::Coord,
        p1: &vgc::coord::Coord,
    ) -> Result<(), String> {
        let builder = self.path_builder.as_mut().unwrap();
        builder.bezier_curve_to(
            Point::new(cp0.x, cp0.y),
            Point::new(cp1.x, cp1.y),
            Point::new(p1.x, p1.y),
        );
        Ok(())
    }

    fn close_shape(&mut self) -> Result<(), String> {
        let fill = self.fill.take().unwrap();
        let path = Path::new(|e| {
            *e = self.path_builder.take().unwrap();
        });

        self.frame.fill(&path, fill);
        Ok(())
    }

    fn end(&mut self) -> Result<(), String> {
        Ok(())
    }
}
