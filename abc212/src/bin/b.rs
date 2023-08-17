use proconio::{input, marker::Chars};

fn main() {
    input! {
        pass: Chars,
    };
    if pass[0] == pass[1] && pass[1] == pass[2] && pass[2] == pass[3] {
        println!("Weak");
        return;
    }
    let mut cnt = 0;
    for i in 0..=2 {
        if (pass[i].to_digit(10).unwrap() + 1) % 10 == pass[i + 1].to_digit(10).unwrap() {
            // println!(
            //     "{}=={}",
            //     pass[i].to_digit(10).unwrap(),
            //     pass[i + 1].to_digit(10).unwrap()
            // );
            cnt += 1;
        }
    }

    if cnt < 3 {
        println!("Strong");
    } else {
        println!("Weak");
    }
}
