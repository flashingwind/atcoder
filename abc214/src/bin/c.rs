use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [u32;n],
        mut t: [u32;n],
    };
    // let mut t = t.to_owned();
    for i in 0..n {
        // println!("T={} T->{i}", t[i]);
        for j_raw in i + 1..n + i + 1 {
            let j = j_raw % n;
            let k = (j_raw - 1) % n;
            let new_t = t[k] + s[k];
            if new_t < t[j] {
                t[j] = new_t;
                // println!("T={} {i}->{j}", t[k] + s[k]);
            } else {
                break;
            }
        }
    }
    for t in t.iter() {
        println!("{}", t);
    }
}
