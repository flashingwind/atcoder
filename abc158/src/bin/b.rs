use proconio::input;

fn main() {
    input! {
        mut n: usize,
        a: usize,
        b: usize,
    };
    let cnt_common = n / (a + b);
    n %= a + b;
    let mut cnt_a = cnt_common * a;
    if n <= a {
        cnt_a += n;
    } else {
        cnt_a += a;
    }
    println!("{}", cnt_a);
}
