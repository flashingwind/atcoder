use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut qs: [(Usize1,Usize1,usize);q],
    };
    qs.sort_by(|a, b| a.2.cmp(&b.2));
    let mut qs2 = vec![Vec::new(); 10];
    qs2[qs[0].2].push((qs[0].0, qs[0].1));
    for i in 1..q {
        if let Some(q_last) = qs2.pop() {
            for q2 in q_last.iter() {
                if q2.2 == qs[i].2 {
                    if q_last.1 >= qs[i].0 && qs[i].1 >= q_last.0 {
                        qs2[qs[i].2].push((qs[i].0.min(q_last.0), qs[i].1.max(q_last.1)));
                    } else {
                        qs2[qs[i].2].push((qs[i].0, qs[i].1));
                    }
                }
            }
        }
        if qs[i].2 == qs[i + 1].2 {}
    }
}
