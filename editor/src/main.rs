
mod scene;
mod utils;
mod toolbars;

use scene::Scene;

use iced::{executor, Renderer, Alignment, Color};
use iced::theme::Theme;

use iced::widget::{column, container, row, button, text};
use iced::window;
use iced::window::icon::from_file_data;
use iced::{Application, Command, Element, Length, Settings};
use toolbars::left::left_toolbar;

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
pub struct VgcEditor {
    scene: Vec<Scene>,
    current_scene: usize,
    show_color_picker: bool,
    pub color_picker: utils::ColorImage,
}

#[derive(Debug, Clone)]
pub enum Message {
    Scene(scene::MsgScene),
    NewEmptyScene,
    
    OpenColorPicker,
    SubmitColor(Color),
    CancelColor,

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
                match self.scene.get_mut(self.current_scene){
                    Some(scene) => scene.update(message),
                    None => println!("No scene"),
                };
            }
            Message::NewEmptyScene => {
                self.current_scene = self.scene.len();
                self.scene.push(Scene::default());
            },

            Message::OpenColorPicker => {
                self.show_color_picker = true;
            },
            Message::SubmitColor(color) => {
                match self.scene.get_mut(self.current_scene){
                    Some(scene) => {
                        self.color_picker.set_color(color);
                        scene.update(scene::MsgScene::SubmitColor(color))
                    },
                    None => println!("No scene"),
                };
                self.show_color_picker = false;
            }
            Message::CancelColor => {
                self.show_color_picker = false;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let current_functionality = match self.scene.get(self.current_scene){
            Some(scene) => &scene.functionality,
            None => &scene::Functionality::None,
        };

        let controls = left_toolbar(self, current_functionality);

        

        let canvas: Element<'_, Message, Renderer<Theme>> = match self.scene.is_empty(){
            true => new_scene(),
            false => self.scene[self.current_scene].view().map(move |message| Message::Scene(message)),
        };

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


 fn new_scene<'a>() -> Element<'a, Message> {
    let text = text("No current scene, click on New Scene to create one!").width(Length::Fixed(150.0));
    let btn_play = button("New Scene")
        .on_press(Message::NewEmptyScene);

    column![text,btn_play]
        .padding(50)
        .spacing(10)
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .into()
}
