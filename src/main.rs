use iced::widget::{button, column, container, image, row, slider, text, text_input, Row, Space};
use iced::{Bottom, Center, ContentFit, Element, Fill, Task, Top};

const APP_NAME: &str = "AccurateRecipe";
const RECIPE_FILE: &str = "recipes.json";

pub fn main() -> iced::Result {
    // Add recipe.json creation if not existend to main file
    iced::application(APP_NAME, AccurateRecipe::update, AccurateRecipe::view)
        .run_with(AccurateRecipe::new)
}

mod cook;

#[derive(Default)]
struct AccurateRecipe {
    page: usize,
    search_value: String,
    portion_multiplier: f32,
    recipes: Vec<cook::Recipe>,
}

#[derive(Debug, Clone)]
enum Message {
    Previous,
    Next,
    SearchChanged(String),
    PortionChanged(f32),
    Search,
}

impl AccurateRecipe {
    fn new() -> (AccurateRecipe, Task<Message>) {
        (
            AccurateRecipe {
                page: 0,
                portion_multiplier: 1.0,
                search_value: String::from(""),
                recipes: cook::recipes_from_file(RECIPE_FILE).unwrap(),
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
                if self.page < self.recipes.len() && self.recipes.len() != 1 {
                    self.page += 1;
                }
            }
            Message::SearchChanged(value) => {
                self.search_value = value;
            }
            Message::Search => {
                println!("Search through recipe hashmap");
                self.search_value.clear();
            }
            Message::PortionChanged(value) => {
                self.portion_multiplier = value;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // TODO: Add portions slider.
        let nav_bar: Row<Message> = row![
            button("←").on_press(Message::Previous),
            text_input("search", &self.search_value)
                .on_input(Message::SearchChanged)
                .on_submit(Message::Search),
            button("→").on_press(Message::Next)
        ];

        let current_recipe: &cook::Recipe = &self.recipes[self.page];
        let body: Row<Message> = row![
            column![
                column![
                    slider(0.5..=10.0, self.portion_multiplier, Message::PortionChanged),
                    Space::new(0, 10),
                    text(current_recipe.portions_multiplied_to_string(self.portion_multiplier)),
                    Space::new(0, 30),
                ],
                container(text(
                    current_recipe.ingredients_multiplied_to_string(self.portion_multiplier)
                ))
                .width(Fill)
                .height(Fill),
                container(text(current_recipe.instructions_to_string()))
                    .width(Fill)
                    .height(Fill)
            ],
            image(&current_recipe.image)
                .width(Fill)
                .content_fit(ContentFit::Fill)
        ];

        let previous_page = if self.page > 0 {
            (self.page - 1).to_string()
        } else {
            0.to_string()
        };
        let next_page = if self.page == (self.recipes.len() - 1) {
            "".to_string()
        } else {
            (self.page + 1).to_string()
        };
        let footer: Row<Message> = row![
            text(previous_page),
            text(&current_recipe.name)
                .align_x(Center)
                .align_y(Center)
                .width(Fill),
            text(next_page)
        ];

        container(row![column![
            nav_bar.align_y(Top).padding(10).width(Fill),
            body.align_y(Center).padding(10),
            footer.align_y(Bottom)
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
fn page_updates_correctly() {
    let recipes = vec![cook::Recipe::default_with_name("test")];

    let mut counter = AccurateRecipe {
        page: 0,
        portion_multiplier: 1.0,
        search_value: String::from("Test"),
        recipes,
    };

    counter.update(Message::Next);
    counter.update(Message::Next);
    counter.update(Message::Previous);

    assert_eq!(counter.page, 1);
}

// TODO: more tests and separating modules.
// Add "Add recipe button", safe to unwrapinitialation function.
