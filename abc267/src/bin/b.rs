use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    s.insert(0, '0');
    let mut slots = vec![false; 7];

    slots[0] |= s[7] == '1';

    slots[1] |= s[4] == '1';

    slots[2] |= s[2] == '1';
    slots[2] |= s[8] == '1';

    slots[3] |= s[1] == '1';
    slots[3] |= s[5] == '1';

    slots[4] |= s[3] == '1';
    slots[4] |= s[9] == '1';

    slots[5] |= s[6] == '1';

    slots[6] |= s[10] == '1';

    // println!("{:?}", slots);
    if s[1] == '0' {
        for i in 0..7 {
            for j in i + 2..7 {
                if slots[i] && slots[j] {
                    // println!("{}~{}:", i + 1, j - 1);
                    for i in i + 1..=j - 1 {
                        if !slots[i] {
                            println!("Yes");
                            return;
                        }
                    }
                }
            }
        }
    }
    println!("No");
}
