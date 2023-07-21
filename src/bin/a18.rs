// Topic: HashMap
//
// Program requirements:
// * Print the name and the number of items in stack for a furniture store
// * If the number of items is 0, print "out of stock" instead if 0
// * The store has:
//   * 5 chairs
//   * 3 beds
//   * 2 tables
//   * 0 couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut furniture_stock: HashMap<&str, i16> = HashMap::new();
    furniture_stock.insert("Chair", 5);
    furniture_stock.insert("Bed", 3);
    furniture_stock.insert("Table", 2);
    furniture_stock.insert("Couch", 0);

    let mut total_items = 0;

    furniture_stock.iter().for_each(|(item, stock)| {
        match stock {
            0 => println!("item: {} stock: out of stock", item),
            _ => println!("item: {} stock: {}", item, stock),
        }
        total_items += stock;
    });

    println!("total stock: {}", total_items);
}
