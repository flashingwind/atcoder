use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ps: [(i64,i64);n],
    };
    for (i, j) in (0..n).tuple_combinations() {
        let f2 = f(ps[i], ps[j]);
        for k in (0..n).filter(|&k| k != i && k != j) {
            if f2(ps[k]) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

fn f((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> impl Fn((i64, i64)) -> bool {
    let a;
    let b;
    let c;
    let d;
    if x2 == x1 {
        a = 1;
        b = -x1;
        c = 0;
        d = 0;
    } else {
        d = x2 - x1;
        a = y2 - y1;
        b = x2 * y1 - x1 * y2;
        c = 1;
    }
    // println!("({x1},{y1})-({x2},{y2}): y={a}x+{b}");
    move |(x, y)| {
        // println!("{}={}", a * x + b, y * c);
        a * x + b == y * c * d
    }
}
