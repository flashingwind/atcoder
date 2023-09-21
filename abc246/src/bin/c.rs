use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: u32,
        x: u32,
        mut a: [u32;n],
    };
    // println!("1 k={k} {:?}", a);
    for i in 0..n {
        // println!("2 a{i}={}-{}*{}={}", a[i], dk, x, a[i] - dk * x);
        let dk = (a[i] / x).min(k);
        a[i] -= dk * x;
        k -= dk;
        if k == 0 {
            break;
        }
    }
    a.sort_unstable();
    a.reverse();
    // println!("2 k={k} {:?}", a);
    if 0 < k {
        for i in 0..n {
            a[i] -= x.min(a[i]);
            k -= 1;
            if k == 0 {
                break;
            }
        }
    }
    // println!("3 k={k} {:?}", a);
    println!("{}", a.iter().fold(0u64, |acc, v| acc + (*v as u64)));
}
