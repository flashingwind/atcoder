use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut aa: [Usize1;k],
    };
    // let mut a = Vec::with_capacity(2 * n + 1);
    let mut a = vec![];
    aa.sort();
    aa.reverse();
    // // println!("{:?}", aa);
    let mut atmp = aa.pop().unwrap();
    for i in 0..n {
        // // println!("{atmp}:{:?}x", a);
        a.push(i);
        if atmp == i {
            atmp = aa.pop().unwrap_or(usize::MAX);
        } else {
            a.push(i);
        }
    }
    // println!("{:?}", a);

    if a.len() == 0 {
        // println!("0");
        return;
    } else if a.len() == 1 {
        // println!("0");
        return;
    }
    let mut sum = 0;
    if a.len() % 2 == 0 {
        for i in (0..a.len()).step_by(2) {
            sum += a[i].abs_diff(a[i + 1]);
            // println!("{}:{},{}:{} sum={sum}", i, a[i], i + 1, a[i + 1]);
        }
    } else {
        let mut tmp = 0;
        for i in (0..=a.len() - 2).step_by(2) {
            tmp += a[i].abs_diff(a[i + 1]);
            // println!("{}:{},{}:{} tmp={tmp}", i, a[i], i + 1, a[i + 1]);
        }
        let mut tmp2 = 0;
        for i in (1..=a.len() - 1).step_by(2) {
            tmp2 += a[i].abs_diff(a[i + 1]);
            // println!("{}:{},{}:{} tmp={tmp2}", i, a[i], i + 1, a[i + 1]);
        }
        sum = tmp.min(tmp2);
    }
    println!("{}", sum);
}
