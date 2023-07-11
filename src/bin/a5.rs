// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through  "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * use a break to exit the loop

fn main() {
    let mut num: i32 = 1;

    loop {
        print!("{} ", num);
        num += 1;

        if num == 5 {
            break;
        }
    }
}