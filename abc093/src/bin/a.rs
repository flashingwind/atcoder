use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut alph = vec!['a', 'b', 'c'];
    for c in s.iter() {
        if let Ok(i) = alph.binary_search(c) {
            alph.remove(i);
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
