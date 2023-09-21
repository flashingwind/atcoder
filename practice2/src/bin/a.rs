use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut g = petgraph::unionfind::UnionFind::new(n);
    for _ in 0..q {
        input! {
            t:u32,
            u:usize,
            v:usize,
        };
        if t == 0 {
            g.union(u, v);
        } else {
            println!("{}", if g.equiv(u, v) { 1 } else { 0 });
        }
    }
}
