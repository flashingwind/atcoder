use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [(u64,u64);n],
    };
    let mut tak = 1u64;
    let mut aok = 1u64;
    for i in 0..n {
        let tmp1;
        if tak % v[i].0 == 0 {
            tmp1 = tak / v[i].0;
        } else {
            tmp1 = tak / v[i].0 + 1;
        }
        let tmp2;
        if aok % v[i].1 == 0 {
            tmp2 = aok / v[i].1;
        } else {
            tmp2 = aok / v[i].1 + 1;
        }
        let t = tmp1.max(tmp2);
        tak = t * v[i].0;
        aok = t * v[i].1;
    }
    println!("{}", tak + aok);
}
