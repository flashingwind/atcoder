use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: u64,
        };
        // let fact = factors(n, &lpf, &lpf_ord, &lpf_pow);
        factors(n);
    }
}

pub fn factors(mut n: u64) {
    let mut cnt = 0;
    let mut i = 2;
    while i * i <= n + 10 {
        // i <= sqrt(n)
        let mut pow_cnt = 0;
        while n % i == 0 {
            n /= i;
            pow_cnt += 1;
        }
        cnt += if pow_cnt == 0 { 0 } else { 1 };
        // println!("{i}^{pow_cnt}: cnt={cnt}");
        if 2 <= cnt {
            println!("Yes");
            // return;
        }
        i += 1;
    }
    println!("No");
}
