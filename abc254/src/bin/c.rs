use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [u32;n],
    };
    if k == 1 {
        println!("Yes");
    }
    let a_sorted = a.iter().map(|v| *v).sorted().collect_vec();
    // println!("{:?}", a);
    // println!("{:?}", a_sorted);
    for i in 0..n - k {
        let mut b = vec![];
        for l in 0..n / k {
            let j = i + k * l;
            if n <= j {
                break;
            }
            b.push(a[j]);
        }
        b.sort();
        b.reverse();
        for l in 0..n / k {
            let j = i + k * l;
            if n <= j {
                break;
            }
            a[j] = b.pop().unwrap();
        }
    }
    if a == a_sorted {
        println!("Yes");
    } else {
        println!("No");
    }
}
