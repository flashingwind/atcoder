use proconio::input;

fn main() {
    input! {
        mut a: usize,
        b: usize,
        k: usize,
    };
    let mut t = 0;
    // println!("{t}:{a}*{k}");
    while a < b {
        a *= k;
        t += 1;
        // println!("{t}:{a}*{k}");
    }
    println!("{}", t);
}
