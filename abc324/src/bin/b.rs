use proconio::input;

fn main() {
    input! {
        n: u64,
    };
    let i_max = 70usize;
    let j_max = 48usize;
    let mut sum = 1;
    for _i in 0..=i_max {
        if u64::MAX / 2 <= sum {
            break;
        } else if n == sum {
            println!("Yes");
            return;
        }
        let mut sum2 = sum;
        for _j in 0..=j_max {
            if u64::MAX / 3 <= sum2 {
                break;
            } else if n == sum2 {
                println!("Yes");
                return;
            }
            sum2 *= 3;
        }
        sum *= 2;
    }
    println!("No")
}
