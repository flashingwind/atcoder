use proconio::input;

fn main() {
    input! {
        is_upper: char,
        t: char,
    };
    if is_upper == 'Y' {
        println!("{}", t.to_ascii_uppercase());
    } else {
        println!("{}", t.to_ascii_lowercase());
    }
}
