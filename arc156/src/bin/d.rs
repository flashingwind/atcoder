use itertools::{iproduct, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u32;n],
    };
    let mut xor = 0;
    let mut index = Vec::new();
    index.push(vec![0].repeat(k));
    for j in (0..n.pow(k)) {
        let p = index.last().unwrap();
        p[0] += 1;

        for k in 0..k {}

        let mut sum = 0;
        for i in pat.iter() {
            sum += a[*i];
            print!("a{}:{} ", *i, a[*i]);
        }
        xor ^= sum;
        println!(" xor={xor}");
    }
    println!("{}", xor);
}
