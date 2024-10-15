use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Recipe {
    pub name: String,
    portions: f32,
    ingredients: HashMap<String, f32>,
    pub instructions: Vec<String>,
    pub image: String,
}

impl Recipe {
    // TODO: Add new funciton
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

/// Read recipes from file with help of serde_json.
pub fn recipes_from_file(filename: &str) -> Result<Vec<Recipe>, std::io::Error> {
    let recipes: Vec<Recipe> = serde_json::from_reader(BufReader::new(File::open(filename)?))?;
    Ok(recipes)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn recipe_with_name(name: &str) -> Recipe {
        Recipe {
            name: name.to_string(),
            portions: 0.0,
            ingredients: HashMap::new(),
            instructions: Vec::new(),
            image: "".to_string(),
        }
    }

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
        let mut recipe = recipe_with_name("ingredients_added_to_recipe");
        recipe.add_ingredient("Flour", 500.0);
        assert!(recipe.ingredients.contains_key("Flour"));
        assert_eq!(recipe.ingredients.get("Flour").unwrap(), &500.0);
    }

    #[test]
    fn ingredients_string_conversion_as_expected() {
        let mut recipe = recipe_with_name("ingredients_string_conversion_as_expected");
        recipe.add_ingredient("Flour", 500.0);
        recipe.add_ingredient("Yeast", 3.0);
        let expected = "Ingredients\n---------------\nFlour: 500 g\nYeast: 3 g\n";
        assert_eq!(&recipe.ingredients_to_string(1.0), expected);
    }

    #[test]
    fn instructions_string_conversion_as_expected() {
        let mut recipe = recipe_with_name("ingredients_added_to_recipe");
        recipe.instructions.push(String::from("Add Plastic."));
        recipe.instructions.push(String::from("Do not eat."));
        let expected = "Instructions\n---------------\n1. Add Plastic.\n2. Do not eat.\n";
        assert_eq!(&recipe.instructions_to_string(), expected);
    }
}
