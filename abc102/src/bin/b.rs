use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
    };
    let mut max_diff = 0;
    for i in 0..n {
        for j in i + 1..n {
            if max_diff < (a[i].abs_diff(a[j])) {
                max_diff = a[i].abs_diff(a[j]);
            }
        }
    }
    println!("{}", max_diff);
}
