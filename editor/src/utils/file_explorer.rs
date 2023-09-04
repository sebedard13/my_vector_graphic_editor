use iced::alignment::Horizontal;
use iced::widget::{button, container, row, text, text_input, Column, Row};
use iced::{Alignment, Element, Length, Renderer, Theme};
use std::fmt::Debug;
use std::path::PathBuf;

use crate::styles;

/// Message for file operations
#[derive(Debug, Clone)]
pub enum Msg {
    InputChange(String),
}

#[derive(Debug, Clone)]
pub enum RtnMsg<T> {
    Own(Msg),
    ToParent(T),
}

#[derive(Default)]
pub enum Action {
    FindExistingFile,
    #[default]
    FindNewFile,
}

pub struct FileExplorerWidget<T>
where
    T: Clone + Debug,
{
    on_search_found: Option<T>,
    on_search_abort: Option<T>,
    pub search_result: Result<PathBuf, String>,

    pub title: Option<String>,
    current_value: String,
    pub action: Action,
    overwrite: bool,
}

impl<T> Default for FileExplorerWidget<T>
where
    T: Clone + Debug,
{
    fn default() -> Self {
        Self {
            search_result: Err(String::from("No search done yet")),
            on_search_found: Option::default(),
            on_search_abort: Option::default(),
            title: Option::default(),
            current_value: String::default(),
            action: Action::default(),
            overwrite: false,
        }
    }
}

impl<T> FileExplorerWidget<T>
where
    T: Clone + Debug,
{
    pub fn update(&mut self, msg: Msg) {
        match msg {
            Msg::InputChange(string) => {
                self.set_value(string);
            }
        }
    }

    pub fn set_value(&mut self, value: String) {
        let path = PathBuf::from(&value.as_str());
        self.current_value = value;
        self.overwrite = false;

        match self.action {
            Action::FindExistingFile => {
                if path.is_dir() {
                    self.search_result = Err(String::from("Path is a directory"));
                    return;
                }

                if !path.is_file() {
                    self.search_result = Err(String::from("File not found"));
                    return;
                }

                self.search_result = Ok(path);
            }
            Action::FindNewFile => {
                if path.is_dir() {
                    self.search_result = Err(String::from("Path is a directory"));
                    return;
                }

                if path.exists() && path.is_file() {
                    self.overwrite = true;
                }

                self.search_result = Ok(path);
            }
        }
    }

    pub fn on_search_found(&mut self, on_search_found: T) {
        self.on_search_found = Some(on_search_found);
    }

    pub fn on_search_abort(&mut self, on_search_abort: T) {
        self.on_search_abort = Some(on_search_abort);
    }

    pub fn view(&self) -> Element<RtnMsg<T>> {
        let text_input: iced::widget::TextInput<'_, RtnMsg<T>, Renderer<Theme>> =
            text_input("", self.current_value.as_str())
                .on_input(|string| RtnMsg::Own(Msg::InputChange(string)));

        let first_row = Row::with_children(vec![
            text("File Name :").width(Length::Shrink).into(),
            text_input.width(Length::Fill).into(),
        ])
        .spacing(5.0)
        .align_items(Alignment::Center)
        .width(Length::Fill);

        let mut btn_open = button("Open").style(styles::btn_normal());
        if let Some(on_press) = self.on_search_found.clone() {
            if self.search_result.is_ok() {
                btn_open = btn_open.on_press(RtnMsg::ToParent(on_press));
            }
        }

        let mut btn_cancel = button("Cancel").style(styles::btn_normal());
        if let Some(on_press) = self.on_search_abort.clone() {
            btn_cancel = btn_cancel.on_press(RtnMsg::ToParent(on_press));
        }

        let error_msg = match self.search_result.clone() {
            Ok(_) => {
                if self.overwrite {
                    "Do you want to overwrite this file?".to_string()
                } else {
                    "".to_string()
                }
            }
            Err(msg) => msg,
        };

        let second_row = container(
            row![text(error_msg), btn_open, btn_cancel]
                .spacing(5.0)
                .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .align_x(Horizontal::Right);

        let mut main_col = Column::new();

        if let Some(title) = self.title.clone() {
            main_col = main_col.push(text(title).size(20));
        }

        main_col = main_col
            .push(first_row)
            .push(second_row)
            .spacing(10.0)
            .padding([10.0; 4])
            .width(Length::Fixed(500.0))
            .height(Length::Fixed(400.0));

        main_col.into()
    }
}
