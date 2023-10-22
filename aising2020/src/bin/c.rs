use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let fs = f(n);
    for t in 1..=n {
        println!("{}", fs[t]);
    }
}

fn f(n: usize) -> Vec<i32> {
    let n_sqrt = n.sqrt() + 1;
    let mut cnt = vec![0; n + 1];
    for x in 1..=n_sqrt {
        for y in 1..=n_sqrt {
            for z in 1..=n_sqrt {
                let v = x * x + y * y + z * z + x * y + y * z + z * x;
                if v <= n {
                    cnt[v] += 1;
                }
            }
        }
    }
    return cnt;
}
