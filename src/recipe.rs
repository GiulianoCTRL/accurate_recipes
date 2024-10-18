use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct Recipe {
    pub name: String,
    portions: f32,
    ingredients: HashMap<String, f32>,
    pub instructions: Vec<String>,
    pub image: String,
}

impl Recipe {
    #[allow(dead_code)]
    pub fn add_ingredient(&mut self, ingredient: &str, weight_in_grams: f32) {
        self.ingredients
            .insert(ingredient.to_string(), weight_in_grams);
    }

    /// Converts ingredients hashmap to string. To be able to update portions prepared
    /// this method takes a multiplier, so that the amount of the ingredients used can
    /// be updated on the fly. Additionally adds Ingredients heading.
    pub fn ingredients_to_string(&self, multiplier: f32) -> String {
        let mut ingredients = String::from("Ingredients\n---------------\n");

        for (k, v) in self.ingredients.iter() {
            let v = *v * multiplier;
            ingredients.push_str(&format!("{k}: {v} g\n"));
        }
        ingredients
    }

    /// Convert instructions to string by prepending their position in the vector,
    /// followed by the instruction. Additionally adds Instructions heading.
    pub fn instructions_to_string(&self) -> String {
        let mut instructions = String::from("Instructions\n---------------\n");

        for (i, s) in self.instructions.iter().enumerate() {
            let i = i + 1;
            instructions.push_str(&format!("{i}. {s}\n"));
        }
        instructions
    }

    pub fn portions_multiplied_to_string(&self, multiplier: f32) -> String {
        let portions = self.portions * multiplier;
        format!("Portions: {portions}")
    }
}

#[allow(dead_code)]
/// Search through slice of recipe and return a vector of matching recipes.
/// For now only the usize of the last returned item is used. Might change once
/// iced supports multiple windows.
pub fn search_recipe_by_name<'a>(recipes: &'a [Recipe], query: &str) -> Vec<(usize, &'a Recipe)> {
    recipes
        .iter()
        .enumerate()
        .filter(|(_, r)| r.name.contains(query))
        .collect()
}

/// Read recipes from file with help of serde_json.
pub fn recipes_from_file(filename: &str) -> Result<Vec<Recipe>, std::io::Error> {
    let recipes: Vec<Recipe> = serde_json::from_reader(BufReader::new(File::open(filename)?))?;
    Ok(recipes)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn recipes_read_correctly() {
        let filename = "test.json";
        let mut ingredients: HashMap<String, f32> = HashMap::new();
        ingredients.insert("test_ingredient".to_string(), 1.0);

        let expected = vec![Recipe {
            name: "test".to_string(),
            portions: 1.0,
            ingredients,
            instructions: vec!["Do not eat.".to_string()],
            image: "images/test.jpg".to_string(),
        }];

        let result = recipes_from_file(filename).unwrap();
        assert_eq!(expected[0], result[0]);
    }

    #[test]
    fn ingredients_added_to_recipe() {
        let mut recipe = Recipe::default();
        recipe.add_ingredient("Flour", 500.0);
        assert!(recipe.ingredients.contains_key("Flour"));
        assert_eq!(recipe.ingredients.get("Flour").unwrap(), &500.0);
    }

    #[test]
    fn ingredients_string_conversion_as_expected() {
        let mut recipe = Recipe::default();
        recipe.add_ingredient("Flour", 500.0);
        let expected = "Ingredients\n---------------\nFlour: 500 g\n";
        assert_eq!(&recipe.ingredients_to_string(1.0), expected);
    }

    #[test]
    fn instructions_string_conversion_as_expected() {
        let mut recipe = Recipe::default();
        recipe.instructions.push(String::from("Add Plastic."));
        recipe.instructions.push(String::from("Do not eat."));
        let expected = "Instructions\n---------------\n1. Add Plastic.\n2. Do not eat.\n";
        assert_eq!(&recipe.instructions_to_string(), expected);
    }

    #[test]
    fn search_returns_correct_results() {
        let mut recipes = vec![Recipe::default(); 5];
        recipes[0].name = String::from("Test123");
        recipes[1].name = String::from("Test222");
        recipes[2].name = String::from("Test333");
        recipes[3].name = String::from("Test456");
        recipes[4].name = String::from("Test112");

        assert_eq!(
            vec![(0usize, &recipes[0]), (4, &recipes[4])],
            search_recipe_by_name(&recipes, "Test1")
        );
    }

    #[test]
    fn unsuccessful_search_returns_empty_vector() {
        let expected: Vec<(usize, &Recipe)> = Vec::new();
        let mut recipes = vec![Recipe::default(); 2];
        recipes[0].name = String::from("Test123");
        recipes[1].name = String::from("Test456");

        assert_eq!(expected, search_recipe_by_name(&recipes, "Skkrrrr"));
    }
}
