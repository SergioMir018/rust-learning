enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let go: Direction = Direction::Up;
    
    match go {
        Direction::Up => println!("going up!"),
        Direction::Down => println!("going down!"),
        Direction::Left => println!("going left!"),
        Direction::Right => println!("going right!")
    }
}