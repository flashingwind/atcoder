use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut qs: [(Usize1,Usize1,char);q],
    };
    qs.sort_by(|a, b| a.2.cmp(&b.2));
    let mut qs2 = vec![];
    for i in 0..q - 1 {
        if qs[i].2 == qs[i + 1].2 {
            if let Some(q_last) = qs2.last() {
                if q_last.2==qs[i].2
            }
            qs2.push((qs[i].0.min(qs[i + 1].0), qs[i].1.max(qs[i + 1].1), qs[i].2))
        }
    }
}
