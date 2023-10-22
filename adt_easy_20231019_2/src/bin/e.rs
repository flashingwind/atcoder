use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64;n],
    };
    a.sort();
    a.reverse();
    let mut max = 0;
    let mut is_ok = false;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if (a[i] + a[j]) % 2 == 0 {
                max = max.max(a[i] + a[j]);
                is_ok = true;
                break;
            }
        }
    }
    if is_ok {
        println!("{}", max);
    } else {
        println!("-1");
    }
}
