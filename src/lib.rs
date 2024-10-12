use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Recipe {
    pub name: String,
    // todo implement Eq for
    portions: f32,
    ingredients: HashMap<String, f32>,
    pub instructions: Vec<String>,
    pub image: String,
}

impl Default for Recipe {
    fn default() -> Self {
        Recipe {
            name: "".to_string(),
            portions: 0.0,
            ingredients: HashMap::new(),
            instructions: Vec::new(),
            image: "".to_string(),
        }
    }
}

impl Recipe {
    #[allow(dead_code)]
    pub fn default_with_name(name: &str) -> Self {
        Recipe {
            name: name.to_string(),
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn add_ingredient(&mut self, ingredient: &str, weight_in_grams: f32) {
        self.ingredients
            .insert(ingredient.to_string(), weight_in_grams);
    }

    pub fn ingredients_multiplied_to_string(&self, multiplier: f32) -> String {
        let mut ingredients = String::from("Ingredients\n---------------\n");

        for (k, v) in self.ingredients.iter() {
            let v = *v * multiplier;
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

    pub fn portions_multiplied_to_string(&self, multiplier: f32) -> String {
        let portions = self.portions * multiplier;
        format!("Portions: {portions}")
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
        image: "images/test.jpg".to_string(),
    }];

    let result = recipes_from_file(filename).unwrap();
    assert_eq!(expected[0], result[0]);
}
