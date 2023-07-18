struct LineItem {
    name: String,
    count: i8
}

fn print_name(name: &str) {
    println!("name: {}", name);
}

fn print_count(count: &i8) {
    println!("count: {}", count);
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 1
        }, 
        LineItem {
            name: String::from("fruit"),
            count: 3
        }
    ];

    for item in receipt {
        print_name(&item.name);
        print_count(&item.count);
        println!("----------");
    }
}
