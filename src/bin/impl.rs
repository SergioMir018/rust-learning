struct Temperature {
    degree_f: f64
}

fn show_temp(temp: Temperature) {
    pritnln!("{:?} dregrees F", temp.degree_f)
}

fn main() {
    let hot = Temperature { degree_f: 99.9 };

    show_temp(hot);
}
