fn main() {
    let mut count: i32 = 10;

    loop {
        print!("{} ", count);
        count = count - 1;

        if count == 0 {
            break;
        }
    }
}