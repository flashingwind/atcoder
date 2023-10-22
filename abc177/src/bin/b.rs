use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut min = t.len();
    for i in 0..(s.len() - t.len() + 1) {
        let mut ok_cnt = 0;
        // println!("i{i}:");
        for j in 0..t.len() {
            if s[i + j] != t[j] {
            } else {
                ok_cnt += 1;
                // println!("s{}:{}=t{j}:{}", i + j, s[i + j], t[j]);
            }
        }
        // println!("err_cnt: {}", err_cnt);
        min = min.min(t.len() - ok_cnt);
    }
    println!("{}", min);
}
