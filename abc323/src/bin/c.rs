use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u32;m],
        s: [Chars;n],
    };
    let mut pt = vec![0; n];

    for (i, ss) in s.iter().enumerate() {
        for (j, &c) in ss.iter().enumerate() {
            if c == 'o' {
                pt[i] += a[j];
            }
        }
        // println!("i:{}", pt[i]);
        pt[i] += 1 + i as u32;
    }
    let max = *pt.iter().max().unwrap();
    let mut max_cnt = 0;
    for (i, ss) in s.iter().enumerate() {
        if max == pt[i] {
            max_cnt += 1;
        }
    }
    let unsorted_j: Vec<usize> = a
        .iter()
        .enumerate()
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .map(|v| v.0)
        .collect();
    a.sort();
    a.reverse();
    // println!("max={max} max_cnt={max_cnt}");
    for i in 0..n {
        let mut cnt = 0;
        if max == pt[i] && max_cnt == 1 {
            println!("0");
            continue;
        }
        for j in 0..m {
            if max < pt[i] {
                break;
            }
            if s[i][unsorted_j[j]] == 'x' {
                if pt[i] <= max {
                    // println!("    +{j}: {}+{}={}", a[j], pt[i], a[j] + pt[i]);
                    pt[i] += a[j];
                    cnt += 1;
                }
            }
        }
        // println!("{}<{}", max, pt[i]);
        println!("{}", cnt);
    }
}
