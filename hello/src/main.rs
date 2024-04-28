fn main() {
    // println! は関数ではなくマクロ
    println!("{:.1}, {:.2}, {:.3}", 3.2, std::f64::consts::PI, 3.2f64.powi(2) * std::f64::consts::PI);
}
