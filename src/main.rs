use std::collections::HashMap;

use iced::widget::{button, column, container, row, text, text_input, Row};
use iced::{Bottom, Center, Element, Fill, Top};

pub fn main() -> iced::Result {
    iced::run(
        "AccurateRecipe",
        AccurateRecipe::update,
        AccurateRecipe::view,
    )
}

#[derive(Default)]
struct AccurateRecipe {
    value: i64,
    input_value: String,
    recipes: Vec<Recipe>,
}

#[derive(Default, Clone, Debug)]
struct Recipe {
    name: String,
    portions: u32,
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
                self.value -= 1;
            }
            Message::Next => {
                self.value += 1;
            }
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::Search => {
                println!("Search through recipe hashmap");
                self.input_value.clear();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let nav_bar: Row<Message> = row![
            button("←").on_press(Message::Previous),
            text_input("search", &self.input_value)
                .on_input(Message::InputChanged)
                .on_submit(Message::Search),
            button("→").on_press(Message::Next)
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

        container(row![column![
            nav_bar.align_y(Top).padding(50),
            body_placeholder.align_y(Center).padding(100),
            footer_placeholder.align_y(Bottom)
        ]
        .padding(20)
        .align_x(Center),])
        .padding(10)
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }
}

#[test]
fn it_counts_properly() {
    let recipes = vec![Recipe {
        name: String::from("test"),
        ..Default::default()
    }];

    let mut counter = AccurateRecipe {
        value: 0,
        input_value: String::from("Test"),
        recipes,
    };

    counter.update(Message::Next);
    counter.update(Message::Next);
    counter.update(Message::Previous);

    assert_eq!(counter.value, 1);
}
