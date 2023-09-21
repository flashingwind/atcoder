use proconio::input;

fn main() {
    input! {
        n: usize,
        mut w: u64,
        mut c: [(u64,u64);n],
    };
    let mut oisi = 0;
    c.sort_by(|a, b| b.0.cmp(&a.0));
    for (i, _) in c.iter().enumerate() {
        if w == 0 {
            break;
        }
        if c[i].1 <= w {
            // println!("c{i}.1={} * c{i}.0={}", c[i].1, c[i].0);
            oisi += c[i].1 * c[i].0;
            w -= c[i].1;
        } else {
            // println!("w={} * c{i}.0={}", w, c[i].0);
            oisi += w * c[i].0;
            w = 0;
        }
    }
    println!("{}", oisi);
}
