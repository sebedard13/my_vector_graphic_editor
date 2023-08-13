mod canvas_camera;
mod move_coord;
mod scene;
mod selected_shape;
mod toolbars;

use scene::Scene;

use iced::executor;
use iced::theme::Theme;

use iced::widget::{column, container, row};
use iced::window;
use iced::window::icon::from_file_data;
use iced::{Application, Command, Element, Length, Settings};
use toolbars::left::{left_toolbar, MsgLeftToolbar};
pub fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

    let icon = from_file_data(include_bytes!("../data/icons/icon.png"), None);

    let icon = match icon {
        Ok(ico) => Some(ico),
        Err(_) => None,
    };
    VgcEditor::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            icon,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct VgcEditor {
    scene: Scene,
}

#[derive(Debug, Clone)]
enum Message {
    Scene(scene::MsgScene),
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

    fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::Scene(message) => {
                self.scene.update(message);
            }
            Message::LeftToolbar(_message) => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let controls = left_toolbar().map(move |message| Message::LeftToolbar(message));

        let canvas = self
            .scene
            .view()
            .map(move |message| Message::Scene(message));

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
