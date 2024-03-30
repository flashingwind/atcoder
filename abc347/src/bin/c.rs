use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        mut d: [usize;n],
    };
    d.sort();
    let d_min = d[0];
    for dd in d.iter_mut() {
        *dd = (*dd - d_min) % (a + b);
        // println!("d={}", dd);
    }
    let e = d.iter().sorted().unique().map(|v| *v).collect_vec();
    for pat in (0..e.len()).permutations(2) {
        if (e[pat[0]].abs_diff(e[pat[1]])) % (a + b) > b {
            //(Ei+1​−Ei​)mod(A+B)>B
            println!("Yes");
            return;
        }
    }
    println!("No");
}
