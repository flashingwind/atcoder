use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: i32,
        q: usize,
        winner: [Usize1;q],
    };
    let mut ps = vec![k - q as i32; n];
    for w in winner.iter() {
        ps[*w] += 1;
    }
    // let p_max = ps.iter().max();
    // println!("{:?}", ps);
    for p in ps.iter_mut() {
        if 0 < *p {
            println!("Yes");
        } else {
            println!("No");
        }
    }
    // println!("{:?}", ps);
}
