use iced::Theme;
use iced::theme::Button;
use iced::{
    widget::button,
    Background, BorderRadius, Color,  Vector,
};


pub fn btn_normal() -> Button{
    Button::Custom(Box::<BtnStyleNormal>::default())
}

#[derive(Default)]
struct BtnStyleNormal {}
impl button::StyleSheet for BtnStyleNormal {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector::default(),
            background: Some(Background::Color(Color::from_rgba8(61, 61, 61, 1.0))),
            border_radius: BorderRadius::from([0.0, 0.0, 0.0, 0.0]),
            border_width: 0.0,
            border_color: Color::default(),
            text_color: Color::from_rgba8(0xEC, 0xEC, 0xEC, 1.0),
        }
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
