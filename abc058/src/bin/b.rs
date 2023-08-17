use proconio::{input, marker::Chars};

fn main() {
    input! {
        odd: Chars,
        eve: Chars,
    };
    let mut passwd = String::from("");
    for (i, c1) in odd.iter().enumerate() {
        passwd.push(*c1);
        if let Some(c2) = eve.get(i) {
            passwd.push(*c2);
        }
    }
    println!("{}", passwd);
}
