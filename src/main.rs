const APP_NAME: &str = "AccurateRecipe";

use accurate_recipe::AccurateRecipe;

pub fn main() -> iced::Result {
    // Add recipe.json creation if not existend to main file
    iced::application(APP_NAME, AccurateRecipe::update, AccurateRecipe::view)
        .run_with(AccurateRecipe::new)
}
