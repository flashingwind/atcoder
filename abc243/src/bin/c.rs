use proconio::{input, marker::Chars};
use std::collections::BTreeMap;

#[warn(clippy::single_match)]
fn main() {
    input! {
        n: usize,
        p: [(usize,usize);n],
        dir: Chars
    };
    //let mut persons: HashMap<u64, Vec<(u64, char)>, Hasher> = HashMap::default();

    let mut to_l_max = BTreeMap::new();
    let mut to_r_min = BTreeMap::new();
    for i in 0..n {
        let (x, y) = p[i];
        if dir[i] == 'R' {
            to_r_min
                .entry(y)
                .and_modify(|x_min| {
                    if x < *x_min {
                        *x_min = x;
                    }
                })
                .or_insert(x);
        } else if dir[i] == 'L' {
            to_l_max
                .entry(y)
                .and_modify(|x_max| {
                    if *x_max < x {
                        *x_max = x;
                    }
                })
                .or_insert(x);
        }
    }
    let y_all = to_r_min.iter().chain(to_l_max.iter());
    for (y, _) in y_all {
        if let Some(x_most_right) = to_l_max.get(y) {
            if let Some(x_most_left) = to_r_min.get(y) {
                if *x_most_left < *x_most_right {
                    println!("Yes");
                    return;
                };
            }
        }
    }
    println!("No");
}
