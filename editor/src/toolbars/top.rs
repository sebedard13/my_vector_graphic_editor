use iced::{
    alignment,
    widget::{button, horizontal_space, row, text},
    Element, Length,
};
use iced_aw::{
    helpers, menu_bar, menu_tree, CloseCondition, ItemHeight, ItemWidth, MenuTree, PathHighlight,
};

use crate::{scene::Functionality, Message, VgcEditor};

pub fn top_toolbar<'a>() -> Element<'a, Message, iced::Renderer> {
    let mb: iced_aw::menu::menu_bar::MenuBar<'_, Message> = menu_bar!(menu_1())
        .item_width(ItemWidth::Uniform(180))
        .item_height(ItemHeight::Uniform(27))
        .spacing(4.0)
        .bounds_expand(30)
        .main_offset(13)
        .cross_offset(16)
        .path_highlight(Some(PathHighlight::MenuActive))
        .close_condition(CloseCondition {
            leave: true,
            click_outside: false,
            click_inside: false,
        });

    row!(mb, horizontal_space(Length::Fill))
        .padding([2, 8])
        .align_items(alignment::Alignment::Center)
        .height(Length::Fixed(100.0))
        .into()
}

fn menu_1<'a>() -> MenuTree<'a, Message, iced::Renderer> {
    helpers::menu_tree(
        labeled_button("File", Message::None),
        vec![
            menu_tree!(
                labeled_button("Save", Message::None)
            ),
            menu_tree!(
                labeled_button("Load", Message::None)
            )
        ],
    )
}

fn labeled_button<'a>(label: &str, msg: Message) -> button::Button<'a, Message, iced::Renderer> {
    base_button(
        text(label)
            .width(Length::Fill)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),
        msg,
    )
}

fn base_button<'a>(
    content: impl Into<Element<'a, Message, iced::Renderer>>,
    msg: Message,
) -> button::Button<'a, Message, iced::Renderer> {
    button(content).padding([4, 8]).on_press(msg)
}
