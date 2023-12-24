use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let a = ['A', 'B', 'C', 'D', 'E'];
    let p1 = a.binary_search(&s[0]).unwrap();
    let p2 = a.binary_search(&s[1]).unwrap();
    let q1 = a.binary_search(&t[0]).unwrap();
    let q2 = a.binary_search(&t[1]).unwrap();
    let mut d1 = p1.abs_diff(p2);
    let mut d2 = q1.abs_diff(q2);
    // println!("d1={d1} d2={d2}");
    if 3 <= d1 {
        d1 = 5 - d1;
    }
    if 3 <= d2 {
        d2 = 5 - d2;
    }
    // println!("d1={d1} d2={d2}");
    if d1 == d2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
