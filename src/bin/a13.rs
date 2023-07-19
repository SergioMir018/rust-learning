// Topic: String
//
// Program requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a person's age, name and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people un a vector
// * Iterate through the vector using a for...in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

use rayon::prelude::*;
struct Person {
    name: String,
    age: i8,
    favorite_color: String,
}

impl Person {
    fn new(name: String, age: i8, favorite_color: String) -> Self {
        Self {
            name,
            age,
            favorite_color,
        }
    }
}

fn print_under_10(persons: Vec<Person>) {
    persons
        .par_iter()
        .filter(|person| person.age < 10)
        .for_each(|person| {
            println!("{} is {}", person.name, person.favorite_color);
        });
}

fn main() {
    let persons = vec![
        Person::new(String::from("John"), 13, String::from("red")),
        Person::new(String::from("Jessica"), 45, String::from("purple")),
        Person::new(String::from("Arnold"), 4, String::from("grey")),
        Person::new(String::from("Luna"), 32, String::from("yellow")),
        Person::new(String::from("Jack"), 8, String::from("orange")),
        Person::new(String::from("Fritz"), 10, String::from("blue")),
        Person::new(String::from("Cynthia"), 7, String::from("pink")),
    ];

    print_under_10(persons);
}