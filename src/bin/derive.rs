/*
* With derive you can use the `Clone` and `Copy` traits to avoid ownership.
* This traits will instead create a new instance of whatever you use it on when * it's passed to a new scope.
*
* Example:
* #[derive(Clone, Copy)]
* struct MyStruct {}
*
* fn new_method(my_struct_instance: Struct) {}
*
* fn main() {
*    let my_struct = MyStruct {};
*
*   new_method(my_struct);
*    new_method(my_struct);
* }
*
* As seen above, the `new_method` function will now have two different
* instances of `MyStruct`, instead of borrowing the same instance.
*/

#[derive(Debug)]
enum Position {
    MANAGER,
    SUPERVISION,
    WORKER,
}

struct Employee {
    position: Position,
    work_hours: i8,
}

impl Employee {
    fn new(position: Position, work_hours: i8) -> Self {
        Self {
            position,
            work_hours,
        }
    }
}

fn main() {
    let me = Employee::new(Position::WORKER, 40);

    println!("{:?}", me.position);
}
