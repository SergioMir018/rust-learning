fn main() {
    let my_name: &str = "Sergio";

    match my_name {
        "Sergio" => println!("Sergio is my name!"),
        "Carlos" => println!("Not my name!"),
        "Alice" => println!("Hello, Alice!"),
        _ => println!("nice to meet you!"),
    }
}