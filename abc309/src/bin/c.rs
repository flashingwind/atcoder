use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [(usize,usize);n],
    };
    let mut sum = 0;
    for i in 0..n {
        sum += a[i].1;
    }
    a.sort_by(|a, b| a.0.cmp(&b.0));

    // println!("{:?}", a);
    if sum <= k {
        println!("1");
        return;
    }

    for i in 0..n {
        if sum < a[i].1 || sum <= k + a[i].1 {
            println!("{}", a[i].0 + 1);
            return;
        }
        sum -= a[i].1;
    }
}
