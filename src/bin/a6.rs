// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Note:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function  must use the enum as a parameter
// * Use a match expression to determine which color name to print

enum Colors {
    Red,
    Blue,
    Yellow,
    Green
}

fn determine_color(color: Colors) {
    match color {
        Colors::Red => print!("Red"),
        Colors::Blue => print!("Blue"),
        Colors::Green => print!("Green"),
        Colors::Yellow => print!("Yellow"),
    }
}

fn main() {
    let color: Colors = Colors::Red;

    determine_color(color);
}