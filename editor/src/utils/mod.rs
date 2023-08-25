use iced::widget::canvas::{Cache, Geometry, Path};
use iced::widget::{canvas, Canvas};
use iced::{mouse, Element, Length};
use iced::{Color, Point, Rectangle, Renderer, Theme};

use crate::Message;

pub struct ColorImage {
    color: Color,
    draw_cache: Cache,
    width: Length,
    height: Length,
}

impl ColorImage {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            width: Length::Fixed(20.0),
            height: Length::Fixed(20.0),
            draw_cache: Cache::default(),
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn set_color(&mut self, color: Color) {
        self.draw_cache.clear();
        self.color = color;
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(self.width)
            .height(self.height)
            .into()
    }
}

impl Default for ColorImage {
    fn default() -> Self {
        Self::new(Color::BLACK)
    }
}

#[derive(Debug, Clone, Default)]
pub struct None {}

impl canvas::Program<Message> for ColorImage {
    type State = None;

    fn draw(
        &self,
        _: &None,
        renderer: &Renderer,
        _: &Theme,
        bounds: Rectangle,
        _: mouse::Cursor,
    ) -> Vec<Geometry> {
        let img = self.draw_cache.draw(renderer, bounds.size(), |frame| {
            let background = Path::rectangle(Point::ORIGIN, frame.size());
            frame.fill(&background, self.color);
        });
        vec![img]
    }

    fn mouse_interaction(
        &self,
        _: &Self::State,
        _: Rectangle,
        _: mouse::Cursor,
    ) -> mouse::Interaction {
        mouse::Interaction::Pointer
    }
}
