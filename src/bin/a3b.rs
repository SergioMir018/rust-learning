// Topic: Flow control using if...else if...else
//
// Program requirements:
// * Display "> 5", "< 5" or "= 5" based on the value of a variable
//
// Notes:
// * Use a variable set to any integer value
// * Use an if...else if...else block to determine which message to display
// * Use the println macro to display the messages to the terminal

fn eval_num_on_five(num: i32) {
    if num > 5 { 
        println!("> 5");
    } else if num < 5 {
        println!("< 5");
    } else if num == 5 {
        println!("= 5");
    }
}

fn main() {
    eval_num_on_five(5);
}