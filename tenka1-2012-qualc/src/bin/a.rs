use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut is_prime = vec![true; n];
    let mut cnt = 0;
    for i in 2..n {
        if is_prime[i] {
            // print!("{i}: ");
            for j in 2..n {
                let m = i * j;
                if n <= m {
                    break;
                }
                // print!("{m}, ");
                is_prime[m] = false;
            }
            cnt += 1;
            // println!();
        }
    }
    // println!("{:?}", is_prime);
    println!("{}", cnt);
}
