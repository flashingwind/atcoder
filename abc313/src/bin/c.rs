use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64;n],
    };
    let min = *a.iter().min().unwrap();
    let max = *a.iter().max().unwrap();
    let mut sum = 0;
    for i in 0..n {
        sum += a[i] - min;
    }
    for i in 0..n {
        a[i] -= sum / 2;
    }
    a.sort();
    println!("{:?}", a);
    println!("{}", sum / 2);
}
