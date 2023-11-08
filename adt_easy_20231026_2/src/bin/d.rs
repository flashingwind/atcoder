use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    };
    s.insert(0, '-');
    for i in 1..=n - 1 {
        let mut max = 0;
        for l in (1..=n - i).rev() {
            let mut is_ok = true;
            for k in 1..=l {
                // println!("s[{}]:{} == s[{}]:{}", k, s[k], k + i, s[k + i]);
                if s[k] == s[k + i] {
                    // println!("break");
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                max = max.max(l);
                break;
            }
        }
        println!("{max}");
    }
}
