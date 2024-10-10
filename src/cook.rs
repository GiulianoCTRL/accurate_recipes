use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Recipe {
    pub name: String,
    // todo implement Eq for
    pub portions: f32,
    pub ingredients: HashMap<String, f32>,
    pub instructions: Vec<String>,
    pub image: String,
}

impl Recipe {
    pub fn ingredients_to_string(&self) -> String {
        let mut ingredients = String::from("Ingredients\n---------------\n");

        for (k, v) in self.ingredients.iter() {
            ingredients.push_str(&format!("{k}: {v} g\n"));
        }
        ingredients
    }

    pub fn instructions_to_string(&self) -> String {
        let mut instructions = String::from("Instructions\n---------------\n");

        for (i, s) in self.instructions.iter().enumerate() {
            let i = i + 1;
            instructions.push_str(&format!("{i}. {s}\n"));
        }
        instructions
    }

    pub fn update_portions(&mut self, multiplier: f32) {
        self.portions *= multiplier;
        for v in self.ingredients.values_mut() {
            *v *= multiplier;
        }
    }
}

pub fn recipes_from_file(filename: &str) -> Result<Vec<Recipe>, std::io::Error> {
    let recipes: Vec<Recipe> = serde_json::from_reader(BufReader::new(File::open(filename)?))?;
    Ok(recipes)
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
        image: "test.jpg".to_string(),
    }];

    let result = recipes_from_file(filename).unwrap();
    assert_eq!(expected[0], result[0]);
}
