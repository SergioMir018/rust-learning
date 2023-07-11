struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

fn main() {
    let my_box: ShippingBox = ShippingBox {
        depth: 4,
        width: 3,
        height: 5,
    };

    let tall: i32 = my_box.height;

    print!("{:?}", tall);
}