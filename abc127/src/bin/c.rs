use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        gates: [(usize,usize);m],
    };
    let mut l_lim = 1;
    let mut r_lim = n;
    for g in gates.iter() {
        l_lim = l_lim.max(g.0);
        r_lim = r_lim.min(g.1);
    }
    if l_lim <= r_lim {
        println!("{}", r_lim - l_lim + 1);
    } else {
        println!("0");
    }
}
