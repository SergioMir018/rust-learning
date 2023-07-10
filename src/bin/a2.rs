// Topic: Basic arithmetics
//
// Program requirements:
// * Use a function to add two number together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result 

fn add_numbers(num1: i32, num2: i32) -> i32 {
     return num1 + num2;
}

fn display_result(num1: i32, num2: i32) {
     print!("{:?}",add_numbers(num1, num2))
}

fn main() {
     display_result(10, 8);
}