use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [[i64;2];n],
    };
    let mut xs = vec![];
    let mut l_min = i64::max_value();
    let mut r_max = i64::min_value();
    for i in 0..n {
        l_min = lr[i][0].min(l_min);
        r_max = lr[i][1].max(r_max);
        xs.push(lr[i][0]);
    }
    if l_min == r_max {
        println!("Yes");
        print!("0");
        for _ in 1..n {
            print!(" 0");
        }
        println!();
        return;
    } else if !(l_min < 0 && 0 < r_max) {
        println!("No");
        return;
    }
    let mut sum: i64 = xs.iter().sum();
    while sum != 0 {
        for i in 0..n {
            if sum < 0 {
                if lr[i][0] <= xs[i] - sum {
                    sum -= sum;
                    xs[i] -= sum;
                } else {
                    sum -= xs[i] - lr[i][0];
                    xs[i] -= xs[i] - lr[i][0];
                }
            } else if 0 < sum {
                if xs[i] + sum <= lr[i][1] {
                    sum += sum;
                    xs[i] += sum;
                } else {
                    sum += lr[i][1] - xs[i];
                    xs[i] += lr[i][1] - xs[i];
                }
            }
            if sum == 0 {
                println!("Yes");
                println!(
                    "{}",
                    xs.iter().map(|v| v.to_string()).collect_vec().join(" ")
                );
                return;
            }
            sum = xs.iter().sum();
        }
    }
    println!("No");
}
