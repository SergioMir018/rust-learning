// Topic: Result
//
// Program requirements:
// * Create a structure named 'Adult' that represents a person aged 21 or elder:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a 'new' function for the 'Adult' structure that returns a Result:
//   * The Ok variant should contain the initialize structure, but only if the person is aged 21 or older
//   *The Err variant should contain a String (or &str) that explains why the structure could not be created
// * Instantiate two 'Adult' structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each 'Adult':
//   * For the Ok variant, print any message you want
//   * For Err variant, print out the error message

struct Adult {
    name: String,
    age: u8
}

impl Adult {
    fn new(name: &str, age: u8) -> Result<Self, (&str, String)> {
        if age >= 21 {
            Ok(
                Self {
                    name: String::from(name),
                    age
                }
            )
        } else {
            Err((name, String::from("Can't create an adult. It's underage")))
        }
    }
}

fn main() {
    let people = vec![
        Adult::new("Sergio", 21),
        Adult::new("Ernesto", 7)
    ];

    for person in people {
        match person {
            Ok(person) => {
                println!("================================");
                println!("Adult created");
                println!("name: {}", person.name);
                println!("age: {}", person.age);
                println!("================================");
            },
            Err((name, e)) => {
                println!("================================");
                println!("Error");
                println!("{}: {}", name,  e);
                println!("================================");
            }
        }
    }
}