// Topic: Implementing functionality with the impl keyword
//
// Program requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Red,
    Blue,
    Yellow
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => print!("color: red"),
            Color::Blue => print!("color: blue"),
            Color::Yellow => print!("color: yellow")
        }
    }
}

struct Dimensions {
    height: i32,
    width: i32,
    depth: i32

}

impl Dimensions {
    fn new_dimensions(height: i32, width: i32, depth: i32) -> Self {
        Self {
            height,
            width,
            depth
        }
    }

    fn print(&self) {
        println!("height: {}", self.height);
        println!("width: {}", self.width);
        println!("depth: {}", self.depth);
    }
}

struct ShippingBox {
    weight: f64,
    color: Color,
    dimensions: Dimensions
}

impl ShippingBox {
    fn new_regular_box(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions
        }
    }

    fn show_characteristics(&self) {
       self.dimensions.print();
       println!("wheight: {:?}", self.weight);
       self.color.print();
    }
}

fn main() {
    let my_box = ShippingBox::new_regular_box(40.5, Color::Red, Dimensions::new_dimensions(30,20,30));

    my_box.show_characteristics();
}
