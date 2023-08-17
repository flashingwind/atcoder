// use fenwick_tree::FenwickTree;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [u64;n],
    };
    // println!("{:?}", a);
    let mut sum = vec![0; n];
    sum[0] = a[0];
    for i in 1..n {
        sum[i] = sum[i - 1] + a[i];
    }
    let mut unproced_p_min = n;
    // println!("{:?}", sum);
    for _ in 0..q {
        input! {
            t: u32,
        };
        if t == 0 {
            input! {
                p: usize,
                x: u64,
            };
            a[p] += x;
            if p < unproced_p_min {
                unproced_p_min = p;
            }
            // println!("add {:?}", a);
            // println!("add {:?}", sum);
        } else {
            input! {
                l: usize,
                r: usize,
            };
            for i in unproced_p_min..n {
                if i == 0 {
                    sum[i] = a[i];
                    continue;
                }
                sum[i] = sum[i - 1] + a[i];
            }
            unproced_p_min = n;

            if l == 0 {
                println!("{}", sum[r - 1]);
            } else {
                println!("{}", sum[r - 1] - sum[l - 1]);
            }
        }
    }

    // let mut tree = FenwickTree::<i32>::with_len(3);
}
