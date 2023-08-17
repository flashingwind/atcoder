use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: Chars,
        b: Chars,
    };
    #[allow(clippy::comparison_chain)]
    if a.len() > b.len() {
        println!("GREATER");
    } else if a.len() < b.len() {
        println!("LESS");
    } else {
        for (i, _) in a.iter().enumerate() {
            if a[i] > b[i] {
                println!("GREATER");
                return;
            } else if a[i] < b[i] {
                println!("LESS");
                return;
            }
        }
        println!("EQUAL");
    }
}
