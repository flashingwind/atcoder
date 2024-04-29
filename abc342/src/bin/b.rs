use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1;n],
        q: usize,
        qs: [(Usize1,Usize1);q],
    };
    // let mut i1 = usize::MAX;
    // let mut i2 = usize::MAX;
    for j in 0..q {
        for (i, pp) in p.iter().enumerate() {
            if qs[j].0 == *pp {
                // i1 = i;
                println!("{}", *pp + 1);
                break;
            } else if qs[j].1 == *pp {
                // i2 = i;
                println!("{}", *pp + 1);
                break;
            }
        }
    }
}
