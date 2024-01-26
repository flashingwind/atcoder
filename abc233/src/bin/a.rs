use proconio::input;

fn main() {
    input! {
        x: f32,
        y: f32,
    };
    if y <= x {
        println!("0");
    } else {
        println!("{}", ((y - x) / 10.).ceil());
    }
}
