enum Discount {
    Percent(i32),
    Flat(i32),
}

struct  Ticket {
    event: String,
    price: i32,
}

fn main() {
    let number = 3;

    match number {
        3 => println!("three"),
        other => println!("number: {}", other)
    }

    let flat = Discount::Flat(2);

    match flat {
        Discount::Flat(2) => println!("flat 2"),
        Discount::Flat(amount) => println!("flat discount of: {}", amount),
        _ => ()
    }

    let ticket = Ticket {
        event: String::from("concert"),
        price: 25
    };

    match ticket {
        Ticket {price: 25, event} => println!("event @ 25: {}", event),
        Ticket {price, ..} => println!("price: {}", price),
    }
}