use proconio::input;

fn main() {
    input! {
        mut yr: usize,
    };
    yr = (yr - 1) / 100 + 1;
    println!("{}", yr);
}
