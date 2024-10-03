use std::collections::HashMap;
use std::path::Path;

use iced::widget::{button, column, container, row, text, text_input, Column, Container, Row};
use iced::{Center, Element, Fill};

pub fn main() -> iced::Result {
    iced::run(
        "A cool counter",
        AccurateRecipe::update,
        AccurateRecipe::view,
    )
}

#[derive(Default)]
struct AccurateRecipe {
    value: i64,
    search_content: String,
}

#[derive(Default, Clone, Debug)]
struct Recipe {
    name: String,
    ingredients: HashMap<String, f64>,
    instructions: Vec<String>,
    picture: String,
}

#[derive(Debug, Clone)]
enum Message {
    Previous,
    Next,
    InputChanged(String),
    Search,
}

impl AccurateRecipe {
    fn update(&mut self, message: Message) {
        match message {
            Message::Previous => {
                self.value += 1;
            }
            Message::Next => {
                self.value -= 1;
            }
            Message::InputChanged(content) => {
                self.search_content = content;
            }
            Message::Search => {
                todo!("Search through recipe hashmap");
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let nav_bar: Row<Message> = row![
            button("<-").on_press(Message::Previous),
            text_input("search", "")
                .on_input(Message::InputChanged)
                .on_submit(Message::Search),
            button("->").on_press(Message::Next)
        ];

        let body_placeholder: Row<Message> =
            row![text(self.value).size(50), text(self.value).size(50)];

        let footer_placeholder: Row<Message> = row![
            button("<<").on_press(Message::Previous),
            text("Recipe Name")
                .height(50)
                .align_x(Center)
                .align_y(Center),
            button(">>").on_press(Message::Next)
        ];

        container(row![
            column![nav_bar, body_placeholder, footer_placeholder]
                .padding(20)
                .align_x(Center),
            row!["Left", "Right"].padding(20).align_y(Center),
        ])
        .padding(10)
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }
}
