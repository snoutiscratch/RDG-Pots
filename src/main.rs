mod potion;
use crate::potion::Potion;
mod inventory;
use crate::inventory::Inventory;
/* --------
    The *janky* droids potion calculator!!
-------- */

fn main() {
    let mut potions: Vec<Potion> = potion::Potion::parse_potions();
    let mut inventory = Inventory::new();

    // CALC //
    inventory.add("accure".to_string(), 2);
    inventory.add("abate".to_string(), 12);
    inventory.add("fervor".to_string(), 14);
    
    
    println!("{}", potions[1]); // calls fmt internally!
}
