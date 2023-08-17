use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };
    let mut sum_a = a[0];
    let mut middle = a[0];
    for k in 0..n {
        let mut max = a[0];
        if k == 0 {
            println!("{}", max + a[0]);
            continue;
        }
        if max < a[k] {
            max = a[k];
        }
        sum_a += a[k];
        middle += sum_a;
        let sum = (k as u64 + 1) * max + middle;
        println!("{}", sum);
    }
}
