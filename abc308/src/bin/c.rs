use num::rational::Rational64;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64,i64);n],
    };
    let mut score = vec![(0usize, Rational64::new(0, 1)); n];
    for i in 0..n {
        let a = ab[i].0;
        let b = ab[i].1;
        score[i] = (i + 1, Rational64::new_raw(a, a + b));
    }
    score.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    // println!("{:?}", score);
    for i in 0..n {
        if i == 0 {
            print!("{}", score[i].0);
        } else {
            print!(" {}", score[i].0);
        }
    }
    println!();
}
