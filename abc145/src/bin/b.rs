use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    if n % 2 == 1 {
        println!("No");
        return;
    }
    let mut s2 = s.iter().skip(n / 2);
    for c1 in s.iter().take(n / 2) {
        let c2 = s2.next().unwrap();
        if c1 != c2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
