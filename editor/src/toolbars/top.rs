use iced::{
    alignment,
    widget::{button, container, horizontal_space, row, text},
    Color, Element, Length, Theme,
};
use iced_aw::{
    helpers, menu_bar,
    style::{self, menu_bar},
    CloseCondition, ItemHeight, MenuTree,
};

use crate::Message;

pub fn top_toolbar<'a>() -> Element<'a, Message, iced::Renderer> {
    let mb: iced_aw::menu::menu_bar::MenuBar<'_, Message> = menu_bar!(menu_file(), menu_export())
        .main_offset(5)
        .padding([0, 0, 0, 0])
        .item_height(ItemHeight::Uniform(26))
        .style(style::MenuBarStyle::Custom(Box::new(MenuBarStyle {})))
        .close_condition(CloseCondition {
            leave: false,
            click_outside: true,
            click_inside: true,
        });

    let content: Element<_> = row!(mb, horizontal_space(Length::Fill))
        .padding([2, 4])
        .align_items(alignment::Alignment::Center)
        .into();

    content
}

fn menu_file<'a>() -> MenuTree<'a, Message, iced::Renderer> {
    helpers::menu_tree(
        menu_bar_button("File", Message::None),
        vec![
            helpers::menu_tree(
                item_button("Save", Message::StartSaveCurrentScene),
                le_vec(),
            ),
            helpers::menu_tree(item_button("Load", Message::StartLoadScene), le_vec()),
        ],
    )
}

fn menu_export<'a>() -> MenuTree<'a, Message, iced::Renderer> {
    helpers::menu_tree(
        menu_bar_button("Export", Message::None),
        vec![
            helpers::menu_tree(item_button("Export as PNG", Message::None), le_vec()),
            helpers::menu_tree(
                item_button("Export as Vector Graphic", Message::None),
                le_vec(),
            ),
        ],
    )
}

fn le_vec() -> Vec<MenuTree<'static, Message, iced::Renderer>> {
    Vec::new()
}

fn menu_bar_button(label: &str, msg: Message) -> button::Button<Message, iced::Renderer> {
    base_button(
        text(label)
            .size(12.0)
            .width(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center)
            .horizontal_alignment(alignment::Horizontal::Center),
        msg,
    )
    .padding([3, 6])
}

fn item_button<'a>(label: &str, msg: Message) -> impl Into<Element<'a, Message, iced::Renderer>> {
    container(
        base_button(
            text(label)
                .size(12.0)
                .width(Length::Fill)
                .height(Length::Fill)
                .vertical_alignment(alignment::Vertical::Center)
                .horizontal_alignment(alignment::Horizontal::Left),
            msg,
        )
        .padding([3, 3])
        .width(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
}

fn base_button<'a>(
    content: impl Into<Element<'a, Message, iced::Renderer>>,
    msg: Message,
) -> button::Button<'a, Message, iced::Renderer> {
    button(content)
        .style(crate::styles::btn_normal())
        .on_press(msg)
}

struct MenuBarStyle;
impl menu_bar::StyleSheet for MenuBarStyle {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> iced_aw::menu::Appearance {
        iced_aw::menu::Appearance {
            background: Color::from_rgba8(0x20, 0x20, 0x20, 1.0),
            border_width: 0.0,
            border_radius: [0.0; 4],
            border_color: Color::from([0.5; 3]),
            background_expand: [0, 4, 0, 4],
            ///Padding of sub menu
            path: Color::from([0.3; 3]),
        }
    }
}
