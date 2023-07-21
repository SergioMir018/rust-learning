use std::collections::HashMap;

#[derive(Debug)]
struct Contents {
    content: String,
}

impl Contents {
    fn new(new_content: &str) -> Self {
        Self {
            content: String::from(new_content),
        }
    }
}

fn main() {
    let mut lockers: HashMap<i32, Contents> = HashMap::new();

    lockers.insert(1, Contents::new("stuff"));
    lockers.insert(2, Contents::new("shirt"));
    lockers.insert(3, Contents::new("book"));

    lockers.iter().for_each(|(key, content)| {
        println!("number: {} content: {:?}", key, content);
    })
}
