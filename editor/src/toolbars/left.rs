use iced::Theme;
use iced::{
    theme::Button,
    widget::{button, column, image, Image},
    Alignment, Background, BorderRadius, Color, Element, Length, Vector,
};

#[derive(Clone, Debug)]
pub enum MsgLeftToolbar {
    MoveCoordCursor,
    // AddCoordCursor,
    // ToggleHandleCursor,
}

pub fn left_toolbar<'a>() -> Element<'a, MsgLeftToolbar> {
    let content = Image::<image::Handle>::new("editor/data/flower.png")
        .width(20)
        .height(16);
    let btn_style = Box::new(BtnStyle::default());
    let btn_play = button(content)
        .on_press(MsgLeftToolbar::MoveCoordCursor)
        .style(Button::Custom(btn_style));

    column![btn_play]
        .padding(2)
        .spacing(5)
        .align_items(Alignment::Start)
        .width(Length::Shrink)
        .into()
}

struct BtnStyle {
    pub appe: button::Appearance,
}

impl Default for BtnStyle {
    fn default() -> Self {
        BtnStyle {
            appe: button::Appearance {
                shadow_offset: Vector::default(),
                background: Some(Background::Color(Color::from_rgba8(61, 61, 61, 1.0))),
                border_radius: BorderRadius::from([0.0, 0.0, 0.0, 0.0]),
                border_width: 0.0,
                border_color: Color::default(),
                text_color: Color::default(),
            },
        }
    }
}

impl button::StyleSheet for BtnStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        self.appe
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            background: Some(Background::Color(Color::from_rgba8(24, 24, 24, 1.0))),
            ..active
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgba8(24, 24, 24, 1.0))),
            shadow_offset: iced::Vector::new(1.0, 1.0),
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: iced::Vector::default(),
            background: active.background.map(|background| match background {
                iced::Background::Color(color) => iced::Background::Color(iced::Color {
                    a: color.a * 0.5,
                    ..color
                }),
                iced::Background::Gradient(gradient) => {
                    iced::Background::Gradient(gradient.mul_alpha(0.5))
                }
            }),
            text_color: iced::Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}
