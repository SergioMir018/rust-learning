// Topic: Data management using tuples
//
// Program requirements:
// * Print whether the y-value of a cartesian coordinate is greater than 5, less than 5 or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if...else block to determine what to print 

fn build_coordinate(value_x: i32, value_y: i32) -> (i32, i32) {
    (value_x, value_y)
}

fn main() {
    let (_, y) = build_coordinate(1, 5);

    if y > 5 {
        print!("The value of coordinate y is greater than 5");
    } else if y < 5 {
        print!("The value of coordinate y is less than 5");
    } else {
        print!("The value of coordinate y is equal to 5");
    }
}