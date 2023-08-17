use proconio::{input, marker::Chars};

fn main() {
    input! {
        str: Chars,
    };
    let mut cnt = 0;
    for i in (0..str.len()).rev() {
        if str[i] == 'a' {
            cnt += 1;
        } else {
            // println!("{}", str[i]);
            break;
        }
    }
    for i in 0..str.len() {
        if 0 < cnt && str[i] == 'a' {
            cnt -= 1;
        } else {
            // println!("{}", str[i]);
            break;
        }
    }
    // println!("cnt={cnt} {}", str.len());
    let mut str2 = "a".repeat(cnt);
    str2.extend(&str);
    // println!("{}", "a".repeat(cnt));
    // println!("{:?}", str2);
    if str2 == str2.chars().rev().collect::<String>() {
        println!("Yes");
    } else {
        println!("No");
    }
}
