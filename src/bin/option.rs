struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

impl Survey {
    fn new(q1: Option<i32>, q2: Option<bool>, q3: Option<String>) -> Self {
        Self {
            q1,
            q2,
            q3
        }
    }
}

fn main() {
    let response = Survey::new(
        Some(12),
        None,
        Some("A".to_owned())
    );

    match response.q1 { 
        Some(q1) => println!("q1 response: {}", q1),
        None => println!("no q1 response")
    }

    match response.q2 { 
        Some(q2) => println!("q2 response: {}", q2),
        None => println!("no q2 response")
    }

    match response.q3 { 
        Some(q3) => println!("q3 response: {}", q3),
        None => println!("no q3 response")
    }
}