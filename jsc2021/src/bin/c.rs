use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    for c in (1..=b).rev() {
        if 2 <= ((b / c) - ((a - 1) / c)) {
            // println!("{}-{}", (b / c), ((a - 1) / c));
            println!("{}", c);
            break;
        }
    }
}
