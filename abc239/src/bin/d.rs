use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    };
    let mut is_all_x_has_p = true;
    for x in a..=b {
        let mut is_p = false;
        for y in c..=d {
            if is_prime(x + y) {
                is_p = true;
                // println!("    {x}+{y} is prime");
            }
        }
        if is_p {
            // println!("{x}+y has prime");
        } else {
            // println!("{x}+y doesn't have prime");
            is_all_x_has_p = false;
        }
    }
    if is_all_x_has_p {
        println!("Aoki")
    } else {
        println!("Takahashi");
    }
}

// https://zenn.dev/toga/books/rust-atcoder/viewer/boolean
fn is_prime(x: u32) -> bool {
    if x < 2 {
        return false;
    }
    for i in 2.. {
        if i * i > x {
            return true;
        }
        if x % i == 0 {
            return false;
        }
    }
    unreachable!();
}
