enum Access {
    Admin,
    Manager,
    User,
    Guest
}

fn main() {
    let access_level = Access::Admin;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false
    };

    if can_access_file {
        print!("the file have been access");
    }
}
