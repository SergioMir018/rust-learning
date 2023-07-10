// Topic: Functions
//
// Program requirements:
// * Display your first name and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display a massage to the terminal

fn first_name() -> String {
    return String::from("Sergio");
}

fn last_name() -> String {
    return String::from("Mir");
}

fn main() {
    println!("{:?} {:?}", first_name(), last_name());
}
