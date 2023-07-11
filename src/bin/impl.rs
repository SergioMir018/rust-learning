struct Temperature {
    degree_f: f64
}

impl Temperature {
    fn frezzing() -> Self {
        Self {
            degree_f: 32.4
        }
    }

    fn show_temp(&self) {
        println!("{:?} dregrees F", self.degree_f)
    }
}

fn main() {
    let hot = Temperature { degree_f: 99.9 };
    let cold = Temperature::frezzing();

    hot.show_temp();
    cold.show_temp();
}
