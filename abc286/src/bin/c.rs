use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: i128,
        b: i128,
        mut s: Chars,
    };
    let mut min_cost = std::i128::MAX;
    let mut kaibun = s.to_owned();
    for t in 0..n {
        // println!("kaibun={:?}", kaibun);
        let mut cost = t as i128 * a;
        for (i, c) in kaibun[0..(n / 2)].iter().enumerate() {
            if *c == kaibun[i] {}
        }
        for (i, _c) in kaibun[0..(n / 2)].iter().enumerate() {
            if kaibun[n - i - 1] != kaibun[i] {
                cost += b;
            }
        }
        // println!("cost={}", cost);
        min_cost = cost.min(min_cost);
        kaibun.rotate_left(1);
    }
    // println!("{:?}", s);
    println!("{}", min_cost);
}
