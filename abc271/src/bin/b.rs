use proconio::{input, marker::Usize1};

fn main() {
    input! {
        num_a: usize,
        num_q: usize,
    };
    let mut a_vec = vec![];
    for i in 0..num_a {
        input! {
            l: usize,
            a: [u32;l],
        };
        a_vec.push(a);
    }
    for t in 0..num_q {
        input! {
            q: Usize1,
            t: Usize1,
        };
        println!("{}", a_vec[q][t]);
    }
}
