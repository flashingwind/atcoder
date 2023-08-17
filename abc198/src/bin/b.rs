use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut zero_cnt = 0;
    let mut str = format!("{}", n);
    for c in str.chars().rev() {
        if c != '0' {
            break;
        } else {
            zero_cnt += 1;
        }
    }

    str = "0".repeat(zero_cnt) + str.as_str();
    if is_kaibun(str) {
        println!("Yes");
    } else {
        println!("No");
    }
}
fn is_kaibun(str: String) -> bool {
    let str_rev = str.chars().rev().collect::<String>();
    for (i, c) in str.chars().enumerate() {
        if c != str_rev.chars().nth(i).unwrap() {
            return false;
        }
        if str.len() / 2 <= i {
            break;
        }
    }
    return true;
}
