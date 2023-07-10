// Topic: Decision making with match
//
// Program requirements:
// * Display "it is true" or "it is false" based on the value of a boolean variable
//
// Notes:
// * Use a variable set to either "true" or "false"
// * Use a match expression to determine which message to display

fn match_bool(v: bool) {
    match v {
        true => println!("it is true"),
        false => println!("it is false"),
    }
}

fn main() {
    let value: bool = false;

    match_bool(value);
}