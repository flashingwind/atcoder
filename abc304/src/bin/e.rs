use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        e: [(usize,usize);m],
        k: usize,
        rules: [(usize,usize);k],
        num_q: usize,
        q: [(usize,usize);num_q],
    };
    let mut g = UnionFind::new(m);
    for (u, v) in e {
        g.union(u - 1, v - 1);
    }
    for i in 0..num_q {
        let x1 = g.find(q[i].0 - 1) + 1;
        let x2 = g.find(q[i].1 - 1) * 1;
        if x1 == x2 {
            println!("No");
            continue;
        }
        let mut is_good = true;
        for r in rules.iter() {
            let u1 = g.find(r.0 - 1) + 1;
            let u2 = g.find(r.1 - 1) + 1;

            if (u1 == x1 && u2 == x2) || (u1 == x2 && u2 == x1) {
                is_good = false;
                break;
            }
        }
        if is_good {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
