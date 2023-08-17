use proconio::input;

fn main() {
    input! {
        word: String,
    };
    if word.ends_with('s') {
        println!("{}es", word);
    } else {
        println!("{}s", word);
    }
}
