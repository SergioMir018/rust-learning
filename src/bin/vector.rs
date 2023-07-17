struct Test {
    score: i32
}

fn main() {
    let my_scores = vec![
        Test { score: 95 },
        Test { score: 90 },
        Test { score: 100 },
        Test { score: 88 }
    ];

    for test in my_scores {
        println!("{}", test.score);        
    }
}
