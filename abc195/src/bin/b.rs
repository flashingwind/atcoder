use proconio::input;

fn main() {
    input! {
        a: f64,//g
        b: f64,//g
        w_kg: f64,//kg
    };
    let w = w_kg * 1000.0;

    if 1.0 <= (w / b).ceil() && (w / b).ceil() <= (w / a).floor() {
        println!("{} {}", (w / b).ceil(), (w / a).floor());
    } else {
        println!("UNSATISFIABLE");
        return;
    }
}
