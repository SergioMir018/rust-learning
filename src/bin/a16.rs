// Topic: Browsing standard library documentation
//
// Program requirements:
// * Print a string un lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library
// * Navigate to the API documentation section
// * Search for functionality to transform  a string (or str) to uppercase and lowercase
// * Try searching for: to_lowercase and to_uppercase

fn main() {
    let word = "hello world";

    println!("{}", word.to_lowercase());
    println!("{}", word.to_uppercase());
}