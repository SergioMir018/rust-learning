// Topic: Ownership
//
// Program requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity
// * Create a function to display the id number

struct GroceryItem {
    id: i32,
    quantity: i32
}

fn display_id(item: &GroceryItem) {
    println!("id: {:?}", item.id)
}

fn display_quantity(item: &GroceryItem) {
    println!("quantity: {:?}", item.quantity)
}

fn main() {
    let my_item = GroceryItem {
        id: 3625,
        quantity: 2
    };

    display_id(&my_item);
    display_quantity(&my_item);
}
