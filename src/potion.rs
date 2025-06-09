use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use serde::Deserialize;
use serde::Serialize;

// Used for storing potions //
#[derive(Debug, Deserialize, Serialize)] // Allows for debug print with :? and to use serde imports.
pub struct Potion {
    pub name: String,
    pub description: String,
    pub recipe: Vec<String>,
}

impl fmt::Display for Potion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "-- [ {} ] --", self.name)?;
        writeln!(f, "{}", self.description);
        writeln!(f, "Recipe: {}", self.recipe.join(" + "));
        Ok(())
    }
}

impl Potion {
    // Importing potions from JSON //
    pub fn parse_potions() -> Vec<Potion> {
        let file = std::fs::read_to_string("potions.json").expect("Failed to read potions.json");

        let data: HashMap<String, Potion> =
            serde_json::from_str(&file).expect("Failed to parse JSON"); // Converts JSON into a hashmap

        data.into_values().collect()
    }
}

