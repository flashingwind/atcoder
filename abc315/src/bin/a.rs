use proconio::input;

fn main() {
    input! {
        mut s: String,
    }
    for c in ['a', 'e', 'i', 'o', 'u'] {
        s.retain(|v| v != c);
    }
    println!("{}", s);
}
