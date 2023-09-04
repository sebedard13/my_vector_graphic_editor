use iced::widget::canvas::Frame;
use iced::widget::canvas::path::Builder;
use vgc::coord::Coord;
use vgc::{VgcRenderer, Rgba};
use iced::widget::canvas::{Fill, Path};
use iced::{Color, Point};

struct IcedFrame<'a>{
    frame: &'a mut Frame,
    fill: Option<Fill>,
    path_builder: Option<Builder>
}

impl IcedFrame<'_> {
   fn new(frame: &mut Frame) -> IcedFrame {
       IcedFrame {
            frame: frame,
            fill: None,
            path_builder: None
       }
   }
}

impl VgcRenderer for IcedFrame<'_> {
    fn create(&mut self, _: u32, _: u32) -> Result<(), String> {
        Ok(())
    }

    fn fill_background(&mut self, _: &Rgba) -> Result<(), String> {
        Ok(())
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

    fn move_curve(&mut self, cp0: &vgc::coord::Coord, cp1: &vgc::coord::Coord, p1: &vgc::coord::Coord) -> Result<(), String> {
        todo!()
    }

    fn close_shape(&mut self) -> Result<(), String> {
        todo!()
    }

    fn end(&mut self) -> Result<(), String> {
        todo!()
    }
}