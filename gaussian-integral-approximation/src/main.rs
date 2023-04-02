fn my_approximation(x: f64) -> f64 {
    let r = 1.0 / ((x * x + 2.0) * (x * x + 1.0));
    if x > 0.0 {
        return -r + 1.0;
    }
    return r;
}

fn main() {
    let mut x = -2.0;
    while x < 2.0 {
        println!("R({}) {}", x, my_approximation(x));
        x += 0.01;
    }
}
