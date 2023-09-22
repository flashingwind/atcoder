use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        p: u64,
        mut f: [u64;n],
    }
    f.sort();
    f.reverse();
    // println!("d={d} f={:?}", f);
    let mut pass_cost = 0;
    let mut f_cost: u64 = f.iter().sum();
    for i in (0..n).step_by(d) {
        let tmp_cost: u64 = f[i..(i + d).min(n)].iter().sum();
        // println!("{i}..{}", (i + d).min(n));
        // println!(
        //     "  f_cost={f_cost}: {} - tmp_cost={tmp_cost}",
        //     (d as u64 * p)
        // );
        if p < tmp_cost {
            pass_cost += p;
            f_cost -= tmp_cost;
        }
        // println!(
        //     "  f_cost={f_cost} + pass_cost={pass_cost} = {}",
        //     pass_cost + f_cost
        // );
    }
    println!("{}", pass_cost + f_cost);
}
