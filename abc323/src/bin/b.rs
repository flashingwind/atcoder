use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
         a: [Chars;n],
    };
    let mut cnt = vec![0; n];

    for (i, s) in a.iter().enumerate() {
        for &c in s.iter() {
            if c == 'o' {
                cnt[i] += 1;
            }
        }
    }
    for (j, (i, _w)) in cnt
        .iter()
        .enumerate()
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .enumerate()
    {
        if j == 0 {
            print!("{}", i + 1);
        } else {
            // println!("{}:{}", i + 1, _w);
            print!(" {}", i + 1);
        }
    }
    println!();
}
