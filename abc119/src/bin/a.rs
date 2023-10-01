use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };
    // let yyyy = s[..=3].to_vec();
    let mm = s[5].to_digit(10).unwrap() * 10 + s[6].to_digit(10).unwrap();
    let dd = s[8].to_digit(10).unwrap() * 10 + s[9].to_digit(10).unwrap();
    if mm < 4 || (mm == 4 && dd <= 30) {
        println!("Heisei");
    } else {
        println!("TBD");
    }
}
