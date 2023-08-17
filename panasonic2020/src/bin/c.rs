use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
    };
    if a.sqrt() + b.sqrt() < c.sqrt() - std::f64::EPSILON {
        println!("Yes");
    } else {
        println!("No");
    }
}
