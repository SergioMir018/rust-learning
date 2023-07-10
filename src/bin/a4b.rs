// Topic: Decision making with march
//
// Program requirements:
// * Display "one", "two", "three" or "other" based ont wether the value of a variable is 1, 2, 3 or some other number
//
// Notes:
// * Use a variable set to any integer
// * use a match expression to determine which message is displayed
// * Use an underscore to match on any type 

fn is_listed(num: i32) {
    match num {
        1 => print!("one"),
        2 => print!("two"),
        3 => print!("three"),
        _ => print!("other"),
    }
}

fn main() {
    let number: i32 = 5;

    is_listed(number);
}