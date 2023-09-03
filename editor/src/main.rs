mod scene;
mod styles;
mod toolbars;
mod utils;

use std::path::PathBuf;

use iced_aw::{modal, Modal};
use scene::Scene;

use iced::theme::Theme;
use iced::{executor, font, Alignment, Color, Renderer};

use iced::widget::{button, column, container, row, text};
use iced::window;
use iced::window::icon::from_file_data;
use iced::{Application, Command, Element, Length, Settings};
use toolbars::left::left_toolbar;
use toolbars::top::top_toolbar;
use utils::file_explorer::{self, FileExplorerWidget};

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

    path_selected: PathBuf,
    file_explorer: FileExplorerWidget<Message>,

    modal: ModalPossible,
}

#[derive(Debug, Clone, Default)]
pub enum Message {
    Scene(scene::MsgScene),
    ChangeSelection,
    NewEmptyScene,

    OpenColorPicker,
    SubmitColor(Color),
    CancelColor,

    FontLoaded(Result<(), font::Error>),

    StartSaveCurrentScene,
    SaveCurrentScene,

    StartLoadScene,
    LoadScene,

    AbortModal,

    #[default]
    None,

    FileExplorerMsg(file_explorer::Msg),
}

#[derive(Debug, Clone, Default)]
pub enum ModalPossible {
    FileExplorer,

    #[default]
    None,
}

impl Application for VgcEditor {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut state = Self { ..Self::default() };

        state.file_explorer.on_search_abort(Message::AbortModal);
        (
            state,
            font::load(iced_aw::graphics::icons::ICON_FONT_BYTES).map(Message::FontLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("VGC Editor")
    }

    fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::Scene(message) => {
                match self.scene.get_mut(self.current_scene) {
                    Some(scene) => {
                        let msg = scene.update(message);
                        msg.iter().for_each(|msg| {
                            let _ = self.update(msg.clone());
                        });
                    }
                    None => println!("No scene"),
                };
            }
            Message::NewEmptyScene => {
                self.current_scene = self.scene.len();
                self.scene.push(Scene::default());
            }

            Message::OpenColorPicker => {
                self.show_color_picker = true;
            }
            Message::SubmitColor(color) => {
                self.show_color_picker = false;
                match self.scene.get_mut(self.current_scene) {
                    Some(scene) => {
                        self.color_picker.set_color(Some(color));
                        let msg = scene.update(scene::MsgScene::SubmitColor(color));
                        msg.iter().for_each(|msg| {
                            let _ = self.update(msg.clone());
                        });
                    }
                    None => {
                        println!("No scene");
                    }
                };
            }
            Message::CancelColor => {
                self.show_color_picker = false;
            }
            Message::FontLoaded(res) => match res {
                Ok(_) => println!("Font loaded"),
                Err(err) => println!("Font error: {:?}", err),
            },
            Message::ChangeSelection => {
                match self.scene.get_mut(self.current_scene) {
                    Some(scene) => {
                        let color_selected = scene.get_color_selected();
                        match color_selected {
                            scene::ColorSelected::None => {
                                //Keep the current color
                            }
                            scene::ColorSelected::MultipleNotSame => {
                                self.color_picker.set_color(None);
                            }
                            scene::ColorSelected::Single(color) => {
                                self.color_picker.set_color(Some(color));
                            }
                        }
                    }
                    None => {
                        println!("No scene");
                    }
                };
            }
            Message::None => println!("None"),
            Message::StartSaveCurrentScene => {
                self.file_explorer.title = Some(String::from("Save Scene"));
                self.file_explorer.search_result = None;
                self.file_explorer.on_search_found(Message::SaveCurrentScene);
                self.modal = ModalPossible::FileExplorer;
            },
            Message::SaveCurrentScene => {
                self.modal = ModalPossible::None;
                match self.scene.get_mut(self.current_scene) {
                    Some(scene) => {}
                    None => {
                        println!("No scene");
                    }
                };
            }
            Message::StartLoadScene => {
                self.file_explorer.title = Some(String::from("Load Scene"));
                self.file_explorer.search_result = None;
                self.file_explorer.on_search_found(Message::LoadScene);
                self.modal = ModalPossible::FileExplorer;
            },
            Message::LoadScene => {
                self.modal = ModalPossible::None;
                if let Some(path) = self.file_explorer.search_result.clone() {
                    println!("Load scene {:?}", path);
                }
            }

            Message::FileExplorerMsg(msg) => {
                self.file_explorer.update(msg);
            }
            Message::AbortModal => {
                self.modal = ModalPossible::None;
            },
        }

        Command::none()
    }

    #[allow(clippy::redundant_closure)] // Because it warn for something we can not correct because it is an enum
    fn view(&self) -> Element<Message> {
        let current_functionality = match self.scene.get(self.current_scene) {
            Some(scene) => &scene.functionality,
            None => &scene::Functionality::None,
        };

        let controls = left_toolbar(self, current_functionality);

        let canvas: Element<'_, Message, Renderer<Theme>> = match self.scene.is_empty() {
            true => new_scene(),
            false => self.scene[self.current_scene]
                .view()
                .map(move |message| Message::Scene(message)),
        };

        let top_toolbar = top_toolbar();

        let content = column![top_toolbar, row![controls, canvas]];

        let modal_over: Option<Element<Message, Renderer<Theme>>> = match self.modal {
            ModalPossible::FileExplorer => {
                Some(self.file_explorer.view().map(move |message| match message {
                    file_explorer::RtnMsg::Own(msg) => Message::FileExplorerMsg(msg),
                    file_explorer::RtnMsg::ToParent(msg) => msg,
                }))
            },
            ModalPossible::None => None,
        };

        modal(
            container(content).width(Length::Fill).height(Length::Fill),
            modal_over,
        )
        .backdrop(Message::AbortModal).on_esc(Message::AbortModal)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn new_scene<'a>() -> Element<'a, Message> {
    let text =
        text("No current scene, click on New Scene to create one!").width(Length::Fixed(150.0));
    let btn_play = button("New Scene").on_press(Message::NewEmptyScene);

    column![text, btn_play]
        .padding(50)
        .spacing(10)
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .into()
}
