use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    s.insert(0, '0');
    let mut cnt_pins = vec![0; 7];

    cnt_pins[0] += s[7].to_digit(10).unwrap();
    cnt_pins[1] += s[4].to_digit(10).unwrap();

    cnt_pins[2] += s[2].to_digit(10).unwrap();
    cnt_pins[2] += s[8].to_digit(10).unwrap();
    cnt_pins[3] += s[1].to_digit(10).unwrap();
    cnt_pins[3] += s[5].to_digit(10).unwrap();
    cnt_pins[4] += s[3].to_digit(10).unwrap();
    cnt_pins[4] += s[9].to_digit(10).unwrap();

    cnt_pins[5] += s[5].to_digit(10).unwrap();
    cnt_pins[6] += s[6].to_digit(10).unwrap();

    if s[1] == '0' {
        for pat in (0..7).combinations(2) {
            if pat[1] - pat[0] == 1 {
                continue;
            }
            if 1 <= cnt_pins[pat[0]] && 1 <= cnt_pins[pat[1]] {
                for i in pat[0] + 1..=pat[1] - 1 {
                    println!("{}~{}~{}:", pat[0] + 1, i, pat[1] - 1);
                    if cnt_pins[i] == 0 {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }
    println!("No");
}
