use crate::recipe;
use crate::Recipe;
use iced::widget::{button, column, container, image, row, slider, text, text_input, Row, Space};
use iced::{Bottom, Center, ContentFit, Element, Fill, Task, Top};
const RECIPE_FILE: &str = "recipes.json";

#[derive(Debug, Clone)]
pub enum Message {
    Previous,
    Next,
    SearchChanged(String),
    PortionChanged(f32),
    Search,
}

#[derive(Default)]
pub struct AccurateRecipe {
    page: usize,
    search_value: String,
    portion_multiplier: f32,
    recipes: Vec<Recipe>,
}

impl AccurateRecipe {
    pub fn new() -> (AccurateRecipe, Task<Message>) {
        (
            AccurateRecipe {
                page: 0,
                portion_multiplier: 1.0,
                search_value: String::from(""),
                recipes: recipe::recipes_from_file(RECIPE_FILE).unwrap(),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Previous => {
                if self.page > 0 {
                    self.page -= 1;
                }
            }
            Message::Next => {
                if self.page < (self.recipes.len() - 1) {
                    self.page += 1;
                }
            }
            Message::SearchChanged(value) => {
                self.search_value = value;
            }
            Message::Search => {
                if let Some((i, _)) =
                    recipe::search_recipe_by_name(&self.recipes, &self.search_value).last()
                {
                    self.page = *i;
                }
                self.search_value.clear();
            }
            Message::PortionChanged(value) => {
                self.portion_multiplier = value;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        // TODO: Add portions slider.
        let nav_bar: Row<Message> = row![
            button("←").on_press(Message::Previous),
            text_input("search", &self.search_value)
                .on_input(Message::SearchChanged)
                .on_submit(Message::Search),
            button("→").on_press(Message::Next)
        ];

        let current_recipe: &Recipe = &self.recipes[self.page];
        let body: Row<Message> = row![
            column![
                column![
                    slider(1.0..=10.0, self.portion_multiplier, Message::PortionChanged),
                    Space::new(0, 10),
                    text(current_recipe.portions_multiplied_to_string(self.portion_multiplier)),
                    Space::new(0, 30),
                ],
                container(text(
                    current_recipe.ingredients_to_string(self.portion_multiplier)
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

#[cfg(test)]
mod tests {

    use super::*;

    fn test_app() -> AccurateRecipe {
        let mut recipes = vec![Recipe::default(); 3];
        recipes[0].name = "Test".to_string();
        AccurateRecipe {
            page: 0,
            portion_multiplier: 1.0,
            search_value: String::from(""),
            recipes,
        }
    }

    #[test]
    fn pages_update_correctly() {
        let mut page_test = test_app();

        page_test.update(Message::Next);
        page_test.update(Message::Next);
        page_test.update(Message::Previous);

        assert_eq!(page_test.page, 1);
    }

    #[test]
    fn search_message_updates_search_query() {
        let mut search_test = test_app();

        let search_value = String::from("test");

        search_test.update(Message::SearchChanged(search_value.clone()));

        assert_eq!(search_test.search_value, search_value.clone());
    }

    // TODO: more tests.
    // Add "Add recipe button", safe to unwrapinitialation function.
}
