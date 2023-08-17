use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i64;n],
    };
    // s_k-s_k-1= a_k
    let mut a = s[0];
    print!("{}", a);
    for (i, _) in s.iter().enumerate().skip(1) {
        a = s[i] - s[i - 1];
        print!(" {}", a);
    }
    println!();
}
