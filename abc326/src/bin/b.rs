use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: u32,
    };

    let mut cs = vec![0; 3];
    for t in n..=919 {
        for c in t.to_string().chars() {
            cs[i] = c.to_digit(10).unwrap();
        }
        let a = cs[0];
        let b = cs[1];
        let c = cs[2];
        println!("t{a}x{b}=?{c}");
        if a * b == c {
            println!("{}", t);
            break;
        }
    }
}
