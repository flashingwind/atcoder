use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars;h],
    };
    let mut ps=vec![];
    for i in 0..h{
        for j in 0..w{
            if s[i][j]=='o'{
                ps.push((i,j));
            }
        }
    }
    println!("{}",ps[0].0.abs_diff(ps[1].0)+ps[0].1.abs_diff(ps[1].1));
}
