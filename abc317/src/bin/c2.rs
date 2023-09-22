use proconio::{input, marker::Usize1};
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut e = vec![vec![0; n]; n];
    for j in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
            c: u32,
        }
        e[a][b] = c;
        e[b][a] = c;
    }
    let mut used = vec![false; n];
    let mut ans = 0;

    for i in 0..n {
        dfs(i, 0, &mut e, &mut used, &mut ans);
    }
    println!("{ans}");
}

fn dfs(v: usize, sum: u32, e: &mut Vec<Vec<u32>>, used: &mut Vec<bool>, ans: &mut u32) {
    used[v] = true;
    *ans = (*ans).max(sum);
    for i in 0..used.len() {
        if !used[i] && e[v][i] != 0 {
            dfs(i, sum + e[v][i], e, used, ans);
        }
    }
    used[v] = false;
}
