// Topic: Advanced matching
//
// * Program requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip and Standard
// * Backstage tickets and Vip tickets include the ticket holder's name
// * All tickets include price
//
// Notes:
// * Use a enum for the tickets with data associated with each variant
// * Create one for each ticket and place into a vector
// * Use a match expression while iterating over the vector to print the ticket info

enum Ticket {
    Standard(i16),
    Backstage(String, i16),
    Vip(String, i16),
}

impl Ticket {
    fn tickets_info(tickets: Vec<Ticket>) {
        for ticket in tickets {
            match ticket {
                Ticket::Standard(price) => println!("Standard price: {}", price),
                Ticket::Backstage(name, price) => println!("Backstage: {} {}", name, price),
                Ticket::Vip(name, price) => println!("Vip: {} {}", name, price),
            }
        }
    }
}


fn main() {
    let spectators = vec![
        Ticket::Standard(50),
        Ticket::Backstage(String::from("Adriana"), 75),
        Ticket::Vip(String::from("John"), 100)
    ];

    Ticket::tickets_info(spectators)
}