#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err(String::from("Invalid input"))
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("{:?}", choice);
}

/// In this function we are taking an input and we will return a void in case Ok or a String in case Err. Then we will
/// call the get_choice method on this input and the return will be assigned to selected_choice. Notice that there's a
/// '?' character at the end of the declaration, this allows us to avoid having to match for the return of the
/// get_choice, thus it will automatically return the Err in case it ocurred, allowing to work with the Ok right after
/// the return 
fn pick_choice(input: &str) -> Result<(), String> {
    let selected_choice = get_choice(input)?;
    print_choice(&selected_choice);
    Ok(())
}

fn main() {
    // let choice = get_choice("start");

    // match choice {
    //     Ok(selected) => print_choice(&selected),
    //     Err(e) => println!("{:?}", e)
    // }

    let err = pick_choice("l");

    println!("{:?}", err);
}
