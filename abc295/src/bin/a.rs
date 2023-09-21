use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String;n],
    };
    for ww in w.iter() {
        if ww == "and" || ww == "not" || ww == "that" || ww == "the" || ww == "you" {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
