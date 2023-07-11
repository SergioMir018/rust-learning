// Topic: Working with expressions
//
// Requirements:
// * Print "it's big" if a variable is > 100
// * Print "it's small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to an expression that determines whether the value is
// >100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine wich message to print

fn eval_on_100(exp: bool) {
    match exp {
        true => print!("it's big"),
        false => print!("it's small")
    }
}

fn main() {
    let number: i32 = 90;
    let eval_exprs: bool = number > 100;

    eval_on_100(eval_exprs);
}
