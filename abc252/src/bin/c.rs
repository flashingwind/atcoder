use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars;n],
    };
    let mut t_min = usize::MAX;
    for stop in 0..=9 {
        let mut ts = vec![usize::MAX; n];
        // println!("{}: ", stop);
        for t in 0..=10 * n {
            let t_index = t % 10;
            for i in 0..n {
                if ts[i] == usize::MAX && s[i][t_index].to_digit(10).unwrap() == stop {
                    ts[i] = t;
                    // println!("  {t}: i={i}");
                    break;
                }
            }
        }
        ts.sort();
        if *ts.last().unwrap() < t_min {
            t_min = *ts.last().unwrap();
        }
        // println!("{}: {}", stop, *ts.last().unwrap());
    }
    println!("{}", t_min);
}
