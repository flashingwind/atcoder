use proconio::input;

fn main() {
    input! {
        l: u32,
    };
    println!("{}", (l as f64).powf(3.) / 27.);
}
