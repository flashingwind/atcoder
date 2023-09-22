use proconio::input;

fn main() {
    input! {
        n: u32,
    }
    println!("{}", f(n));
}

fn f(k: u32) -> u32 {
    if k == 0 {
        return 1;
    } else {
        return k * f(k - 1);
    }
}
