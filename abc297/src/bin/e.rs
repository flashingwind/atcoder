use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [u64;n],
    };
    let mut prices = Vec::new();
    let mut cnt = vec![0; n];
    let mut cnt_max = vec![0; n];
    a.sort();
    for i in 0..n - 1 {
        cnt_max[i] = a[i + 1] / a[i];
    }
    while prices.len() < k {
        let mut price = 0;
        let mut carry = true;
        for i in 0..n - 1 {
            if carry {
                carry = false;
            } else {
                break;
            }
            if cnt[i] < cnt_max[i] {
                price += a[i];
                cnt[i] += 1;
                if !prices.contains(&price) {
                    prices.push(price);
                }
                // println!("i={i} cnt={:?}", cnt);
            }
            if cnt[i] == cnt_max[i] {
                cnt[i] = 0;
                cnt[i + 1] += 1;
                carry = true;
            }
        }
        // println!("prices={:?}", prices);
    }
    println!("{}", prices.pop().unwrap());
}
