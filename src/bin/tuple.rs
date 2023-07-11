fn example_tuple() -> (i32, i32, i32) {
    (1, 2, 3)
}

fn main() {
    let (a, b, c) = example_tuple();
    let numbers = example_tuple();
    
    println!("destruct: {:?} index: {:?}", a, numbers.0);
    println!("destruct: {:?} index: {:?}", b, numbers.1);
    println!("destruct: {:?} index: {:?}", c, numbers.2);
}