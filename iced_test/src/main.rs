//! This example showcases an interactive version of the Game of Life, invented
//! by John Conway. It leverages a `Canvas` together with other widgets.
mod grid;
mod toolbars;

use grid::Grid;

use iced::executor;
use iced::theme::Theme;

use iced::widget::{column, container, row};
use iced::window;
use iced::{Application, Command, Element, Length, Settings};
use toolbars::left::{left_toolbar, MsgLeftToolbar};

pub fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

    VgcEditor::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct VgcEditor {
    grid: Grid,
}

#[derive(Debug, Clone)]
enum Message {
    Grid(grid::Message, usize),
    LeftToolbar(MsgLeftToolbar),
}

impl Application for VgcEditor {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self { ..Self::default() }, Command::none())
    }

    fn title(&self) -> String {
        String::from("VGC Editor")
    }

    fn update(&mut self, _: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let controls = left_toolbar().map(move |message| Message::LeftToolbar(message));

        let canvas = self
            .grid
            .view()
            .map(move |message| Message::Grid(message, 1));

        let top_toolbar = container(row![])
            .width(Length::Fill)
            .height(Length::Fixed(50.0));

        let content = column![top_toolbar, row![controls, canvas]];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
