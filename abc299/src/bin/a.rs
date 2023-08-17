use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    };
    let mut bar_cnt = 0;
    let mut str_is_after_bar = false;
    for c in s.iter() {
        if *c == '|' {
            bar_cnt += 1;
            if bar_cnt == 2 && str_is_after_bar {
                println!("in");
                return;
            }
        } else if bar_cnt == 1 && *c == '*' {
            str_is_after_bar = true;
        }
    }
    println!("out");
}
