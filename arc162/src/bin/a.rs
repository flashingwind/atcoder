use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            p: [Usize1;n],
        };
        // println!("{:?}", p);
        let mut cand = Vec::new();
        for i in 0..n {
            let pp: Vec<_> = p
                .iter()
                .enumerate()
                .filter(|&(j, pj)| i < j && *pj < p[i])
                .collect();

            if pp.len() == 0 {
                // println!("{:?}", pp);
                cand.push(i);
            }
        }
        // println!("{:?}", cand);
        if 0 < cand.len() {
            println!("{}", cand.len());
        } else {
            println!("{}", p.len());
        }
    }
}
