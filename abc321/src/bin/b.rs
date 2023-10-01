use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        mut a: [u64;n-1],
    };
    a.sort();
    let min = a.remove(0);
    let max = a.pop().unwrap();
    let mut sum: u64 = a.iter().sum::<u64>();
    if x <= sum {
        println!("{}", 0);
    } else {
        for new in 0..=max + 1 {
            if new < min {
                sum = a.iter().sum::<u64>() + min;
            } else if max < new {
                sum = a.iter().sum::<u64>() + max;
            } else if min <= new && new <= max {
                sum = a.iter().sum::<u64>() + new;
            }
            if x <= sum {
                println!("{}", new);
                return;
            }
        }
        println!("-1");
        return;
    }
}
