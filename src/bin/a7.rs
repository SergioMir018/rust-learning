// Topic: Organizing similar data using structs
//
// Program requirements:
// * Print the favor of a drink and it's fluid ounces
//
// Notes:
// * Use a enum to create different flavors of drinks
// * Use a struct to store the drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavors {
    Strawberry,
    Chocolate,
    Mango,
    Banana
}

impl std::fmt::Display for Flavors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flavors::Strawberry => write!(f, "Strawberry"),
            Flavors::Chocolate => write!(f, "Chocolate"),
            Flavors::Mango => write!(f, "Mango"),
            Flavors::Banana => write!(f, "Banana"),
        }
    }
}

struct Drink {
    flavor: Flavors,
    fluid_ounces: f64
}

fn match_drink(drink: Drink) {
    println!("{} {}", drink.flavor, drink.fluid_ounces);
    match drink.flavor {
        Flavors::Strawberry => println!("This drink is strawberry flavored!"),
        Flavors::Chocolate => println!("This drink is chocolate flavored!"),
        Flavors::Mango => println!("This drink is mango flavored!"),
        Flavors::Banana => println!("This drink is banana flavored!"),
    }
}

fn main() {
    let drink1: Drink = Drink {
        flavor: Flavors::Banana,
        fluid_ounces: 70
    };
    let drink2: Drink = Drink {
        flavor: Flavors::Mango,
        fluid_ounces: 89
    };
    let drink3: Drink = Drink {
        flavor: Flavors::Chocolate,
        fluid_ounces: 100
    };
    let drink4: Drink = Drink {
        flavor: Flavors::Strawberry,
        fluid_ounces: 60
    };

    match_drink(drink1);
    match_drink(drink2);
    match_drink(drink3);
    match_drink(drink4);
}