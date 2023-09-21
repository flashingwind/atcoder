use proconio::input;

fn main() {
    input! {
        k: u32,
        // a_chars: Chars,
        // b_chars: Chars,
        a: String,
        b: String,
    };
    // let mut a = 0u64;
    // for c in a_chars.iter() {
    //     a *= k;
    //     a += c.to_digit(10);
    // }
    // let mut b = 0u64;
    // for c in b_chars.iter() {
    //     b *= k;
    //     b += c.to_digit(10);
    // }
    println!(
        "{}",
        u64::from_str_radix(&a, k).unwrap() * u64::from_str_radix(&b, k).unwrap()
    );
}
