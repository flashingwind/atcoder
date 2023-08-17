use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut exp: Vec<String> = Vec::new();
    let mut buf = String::new();
    for d in s {
        if d == '+' {
            exp.push(buf.to_owned());
            buf.clear();
        } else {
            buf.push(d);
        }
    }
    exp.push(buf.to_owned());
    let mut zero_cnt = 0;
    for e in exp.iter() {
        let mut is_include_zero = false;
        for num in e.split('*') {
            if num == "0" {
                is_include_zero = true;
                break;
            }
        }
        if !is_include_zero {
            zero_cnt += 1;
        }
    }
    println!("{}", zero_cnt);
}
