use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use iced::widget::{button, column, container, image, row, text, text_input, Row};
use iced::{Bottom, Center, Element, Fill, Task, Top};
use serde::{Deserialize, Serialize};

const APP_NAME: &str = "AccurateRecipe";
const RECIPE_FILE: &str = "recipes.json";

pub fn main() -> iced::Result {
    // Add recipe.json creation if not existend to main file
    iced::application(APP_NAME, AccurateRecipe::update, AccurateRecipe::view)
        .run_with(AccurateRecipe::new)
}

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Recipe {
    name: String,
    portions: u32,
    ingredients: HashMap<String, u32>,
    instructions: Vec<String>,
    image: String,
}

impl Recipe {
    fn ingredients_to_string(&self) -> String {
        todo!()
    }
    fn instruction_to_string(&self) -> String {
        todo!()
    }
}
fn recipes_from_file(filename: &str) -> Result<Vec<Recipe>, std::io::Error> {
    let recipes: Vec<Recipe> = serde_json::from_reader(BufReader::new(File::open(filename)?))?;
    Ok(recipes)
}

#[derive(Default)]
struct AccurateRecipe {
    page: usize,
    input_value: String,
    recipes: Vec<Recipe>,
}

#[derive(Debug, Clone)]
enum Message {
    Previous,
    Next,
    InputChanged(String),
    Search,
}

impl AccurateRecipe {
    fn new() -> (AccurateRecipe, Task<Message>) {
        (
            AccurateRecipe {
                page: 0,
                input_value: String::from(""),
                recipes: recipes_from_file(RECIPE_FILE).unwrap(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Previous => {
                if self.page > 0 {
                    self.page -= 1;
                }
            }
            Message::Next => {
                if self.page < self.recipes.len() {
                    self.page += 1;
                }
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

        let body_placeholder: Row<Message> = row![
            column![
                container(column!["Ingredients", "lorem ipsum"])
                    .width(Fill)
                    .height(Fill),
                container(column!["Instructions", "lorem ipsum"])
                    .width(Fill)
                    .height(Fill)
            ],
            image(&self.recipes[self.page].image).width(Fill)
        ];

        let previous_page = if self.page > 0 {
            (self.page - 1).to_string()
        } else {
            0.to_string()
        };
        let next_page = if self.page == self.recipes.len() {
            "end".to_string()
        } else {
            (self.page + 1).to_string()
        };
        let footer_placeholder: Row<Message> = row![
            text(previous_page),
            text(&self.recipes[self.page].name)
                .align_x(Center)
                .align_y(Center)
                .width(Fill),
            text(next_page)
        ];

        container(row![column![
            nav_bar.align_y(Top).padding(10).width(Fill),
            body_placeholder.align_y(Center).padding(10),
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
        page: 0,
        input_value: String::from("Test"),
        recipes,
    };

    counter.update(Message::Next);
    counter.update(Message::Next);
    counter.update(Message::Previous);

    assert_eq!(counter.page, 1);
}

#[test]
fn recipes_read_correctly() {
    let filename = "test.json";
    let mut ingredients: HashMap<String, u32> = HashMap::new();
    ingredients.insert("test_ingredient".to_string(), 1);

    let expected = vec![Recipe {
        name: "test".to_string(),
        portions: 1,
        ingredients,
        instructions: vec!["Do not eat.".to_string()],
        image: "test.jpg".to_string(),
    }];

    let result = recipes_from_file(filename).unwrap();
    assert_eq!(expected[0], result[0]);
}
