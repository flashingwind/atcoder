use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p_of: [Usize1;n],
    }
    let mut c_of = vec![vec![]; n];
    for i in 0..n {
        c_of[p_of[i]].push(i);
    }
    for i in 0..n {
        println!("{}", children_cnt);
    }
}
fn c_cnt() {
    let mut children_cnt = vec![None; n];
    for (i, children) in c_of.iter().enumerate() {
        if children.len() == 1 {
            children_cnt[i] = Some(1);
        } else {
            for node in children.iter() {
                children_cnt[i] += children_cnt[node];
            }
        }
    }
}
