use std::collections::HashMap;
use crate::potion::Potion;

#[derive(Debug)]
pub struct Inventory {
    pub items: HashMap<String, u16>
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            items: HashMap::new(), // Rust structs need constructors
        }
    }

    // Management //
    pub fn add(&mut self, name: String, count: u8) {
        if let Some(current) = self.items.get_mut(&name) {
            *current += count as u16;
        } else {
            self.items.insert(name, count as u16);
        }
    }

    // Calculates potions given inventory and potions
    pub fn craftable<'a>(&self, potions: &'a Vec<Potion>) -> Vec<&'a Potion> {
        let mut craftable = Vec::new();

        for potion in potions {
            let mut can_craft = true;

            // checking recipes against inventory
            for berry in &potion.recipe {
                // Check if it can't be crafted
                let count = self.items.get(berry);
                if count.is_none() || *count.unwrap() == 0 {
                    can_craft = false;
                    break;
                }
            }

            // Only push the potion if it can be crafted
            if can_craft {
                craftable.push(potion);
            }
        }

        craftable
    }
}