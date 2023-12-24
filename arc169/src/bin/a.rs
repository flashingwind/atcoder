use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [i32;n],
        mut p: [Usize1;n - 1],
    };
    let mut b = vec!["X".to_string(); n];
    p.insert(0, 0);
    for i in 1..n {
        if a[i] == 0 {
            if a[p[i]] < 0 {
                b[p[i]] = "-".to_string();
            } else if 0 < a[p[i]] {
                b[p[i]] = "+".to_string();
            } else {
                b[p[i]] = "0".to_string();
            }
        } else if a[i] < 0 {
            b[p[i]] = "-".to_string();
        } else if 0 < a[i] {
            b[p[i]] = "+".to_string();
        } else {
            b[p[i]] = "0".to_string();
        }
    }

    if b[0] == "X" {
        println!("{}", a[0]);
    } else {
        println!("{}", b[0]);
    }
}
