use iced::Theme;
use iced::{
    theme::Button,
    widget::{button, column, image, Image},
    Alignment, Background, BorderRadius, Color, Element, Length, Vector,
};
use iced_aw::color_picker;

use crate::scene::{Functionality, MsgScene};
use crate::{Message, VgcEditor};

pub fn left_toolbar<'a>(
    vgc_editor: &'a VgcEditor,
    current_functionality: &Functionality,
) -> Element<'a, Message> {
    let btn_move = {
        let btn_style = match current_functionality {
            Functionality::MoveCoord(..) => Button::Custom(Box::<BtnStyleSelected>::default()),
            _ => crate::styles::btn_normal(),
        };

        let img = Image::<image::Handle>::new("editor/data/arrow_pointer.png")
            .width(20)
            .height(20);

        button(img)
            .on_press(Message::Scene(MsgScene::ChangeFunctionality(
                Functionality::MoveCoord_default(),
            )))
            .style(btn_style)
    };

    let btn_pen = {
        let btn_style = match current_functionality {
            Functionality::CreateOrAddPoint
            | Functionality::CreateNextPoint
            | Functionality::RemovePoint => Button::Custom(Box::<BtnStyleSelected>::default()),
            _ =>  crate::styles::btn_normal(),
        };
        let img = Image::<image::Handle>::new("editor/data/pen_nib.png")
            .width(20)
            .height(20);

        button(img)
            .on_press(Message::Scene(MsgScene::ChangeFunctionality(
                Functionality::CreateOrAddPoint_default(),
            )))
            .style(btn_style)
    };

    let btn_bend_tools = {
        let btn_style = match current_functionality {
            Functionality::SeparateHandle => Button::Custom(Box::<BtnStyleSelected>::default()),
            _ =>  crate::styles::btn_normal(),
        };

        let img = Image::<image::Handle>::new("editor/data/bezier_curve.png")
            .width(20)
            .height(20);

        button(img)
            .on_press(Message::Scene(MsgScene::ChangeFunctionality(
                Functionality::MoveHandle_default(),
            )))
            .style(btn_style)
    };

    let color_picker = {
        let but = button(vgc_editor.color_picker.view())
            .on_press(Message::OpenColorPicker)
            .style(crate::styles::btn_normal());

        let color = match vgc_editor.color_picker.get_color() {
            Some(c) => c,
            None => Color::from_rgba8(0, 0, 0, 1.0),
        };

        color_picker(
            vgc_editor.show_color_picker,
            color,
            but,
            Message::CancelColor,
            Message::SubmitColor,
        )
    };

    column![btn_move, btn_pen, btn_bend_tools, color_picker]
        .padding(2)
        .spacing(5)
        .align_items(Alignment::Start)
        .width(Length::Shrink)
        .into()
}

struct BtnStyleSelected {
    pub appe: button::Appearance,
}

impl Default for BtnStyleSelected {
    fn default() -> Self {
        BtnStyleSelected {
            appe: button::Appearance {
                shadow_offset: Vector::default(),
                background: Some(Background::Color(Color::from_rgba8(0xD5, 0x58, 0x14, 1.0))),
                border_radius: BorderRadius::from([0.0, 0.0, 0.0, 0.0]),
                border_width: 0.0,
                border_color: Color::default(),
                text_color: Color::default(),
            },
        }
    }
}

impl button::StyleSheet for BtnStyleSelected {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        self.appe
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
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

