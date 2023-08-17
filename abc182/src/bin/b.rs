use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [u64;n],
    };
    let mut prms = Vec::new();
    for a in arr.iter() {
        prms.extend(factorization(*a));
    }
}

fn factorization(n: u64) -> Vec<u64> {
    fn factor_sub(n: u64, m: u64) -> (u64, u64) {
        let mut c = 0;
        let mut x = n;
        while x % m == 0 {
            c += 1;
            x /= m;
        }
        (c, x)
    }

    let (c, n) = factor_sub(n, 2);
    if c > 0 {
        return Vec::from([2, c]);
    }
    let mut x = 3;
    let mut m = n;
    while x * x <= m {
        let (c, n) = factor_sub(m, x);
        if c > 0 {
            return Vec::from([x, c]);
        }
        m = n;
        x += 2;
    }
    if m > 1 {
        return Vec::from([m, 1]);
    }
    return Vec::from([1, 1]);
}
