use proconio::input;

fn main() {
    input! {
        n: usize,
        tpx: [(u64,u64,u64);n],
    };
    let mut min_p = u64::MAX;
    for i in 0..n {
        if tpx[i].2 > tpx[i].0 {
            min_p = min_p.min(tpx[i].1);
        }
    }
    if min_p == u64::MAX {
        println!("-1");
    } else {
        println!("{}", min_p);
    }
}
