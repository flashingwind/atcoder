use proconio::input;

fn main() {
    input! {
        x: usize,
    };
    println!("{}", get_prime(x));
}

fn get_prime(x: usize) -> usize {
    let n = x + 86; // p(14,357番目)=155,921	、pi+1-pi=86
    let mut is_prime = vec![true; n + 1];

    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n / 2 {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    for (i, _) in is_prime.iter().enumerate().filter(|a| x <= a.0 && *a.1) {
        // println!("i={}", i);
        return i;
    }
    return 0;
}
