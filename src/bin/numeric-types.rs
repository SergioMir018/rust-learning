fn sub(a: i32, b: i32) -> i32 {
     return a - b;
}

fn main() {
     let _sum: i32 = 2 + 2;
     let _value: i32 = 10 - 5;
     let _division: i32 = 10 / 2;
     let _multiplication: i32 = 6 * 6;
     let _rem: i32 = 6 % 4;

     let five: i32 = sub(8 , 5);

     println!("{:?}", five);
}