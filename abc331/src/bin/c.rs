use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        a: [usize;n],
    };
    let i_cnt = BTreeMap::new();
    for i in 0..n {
        *i_cnt.entry(i).or_insert(0) += 1;
    }
    let sorted_a = a.iter().map(|v| *v).sorted().collect_vec();

    // println!("sorted_a={:?}", sorted_a);
    let mut sum_by_a = vec![0usize; sorted_a[n - 1] + 1];
    let mut sum = 0;
    for i in sorted_a.iter() {
        sum += sorted_a[i + 1];
        if sorted_a[i] != sorted_a[i + 1] {
            sum_by_a[sorted_a[i]] += sum;
        }
    }

    // println!("sum_by_a={:?}", sum_by_a);
    for i in 0..n {
        if i == 0 {
            print!("{}", sum_by_a[a[i]]);
        } else {
            print!(" {}", sum_by_a[a[i]]);
        }
    }
    println!();
}
