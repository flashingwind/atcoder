use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let mut lpf: Vec<_> = (0..=10u32.pow(8)).collect();
    //---
    {
        let n = 10u32.pow(8);
        for i in 2..=n {
            for j in i..=n / i {
                if lpf[(i * j) as usize] == i * j {
                    lpf[(i * j) as usize] = i;
                }
            }
        }
    }
    let mut lpf_ord = vec![0; n + 1];
    let mut lpf_pow = vec![1; n + 1];
    for i in 2..=n as usize {
        let j = i / lpf[i] as usize;
        lpf_ord[i] = if lpf[i] == lpf[j as usize] {
            lpf_ord[j] + 1
        } else {
            1
        };
        lpf_pow[i] = if lpf[i] == lpf[j] {
            lpf_pow[j] * lpf[i]
        } else {
            lpf[i]
        };
    }
    //---
    for _ in 0..t {
        input! {
            n: u32,
        };
        proc(n, &mut lpf);
    }
}
fn proc(n: u32, lpf: &[u32], lpf_ord: &[u32], lpf_pow: &[u32]) {
    // `n` の素因数(重複あり)
    let fact = factors(n, lpf, lpf_ord, lpf_pow);
    for bits in 0usize..(1 << fact.len()) {
        let v = (0..fact.len())
            .filter(|x| (bits & (1 << x)) != 0)
            .map(|i| fact[i]);
        let sum: u32 = v.sum();
        if sum == n {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

/// `n` の素因数分解を返す。
fn factors(mut n: u32, lpf: &[u32], lpf_ord: &[u32], lpf_pow: &[u32]) -> Vec<(usize, u32)> {
    let mut res = vec![];
    while n > 1 {
        res.push((lpf[n as usize], lpf_ord[n as usize]));
        n /= lpf_pow[n as usize];
    }
    res
}
