use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut es = Vec::new();
    for i in 0..n - 1 {
        for j in i + 1..n {
            input! {
                tmp: u64,
            }
            es.push((tmp, i, j));
        }
    }
    let m = es.len();
    let mut pat = (0..n).into_iter().collect();
    let mut max = dfs(pat, es, 0);
    println!("{}", max);
}

fn dfs(pat: Vec<usize>, es: Vec<(u64, usize, usize)>, max: u64) -> u64 {
    let mut is_visited = vec![false; n];
    let mut sum = 0;
    // println!("{:?}", pat);
    for (i, &v) in pat.iter().enumerate() {
        let val = es[v].0;
        let i = es[v].1;
        let j = es[v].2;
        if !is_visited[i] && !is_visited[j] {
            sum += val;
            // println!("  +{i},{j}:{val}");
        } else {
            // println!("  -{i},{j}:{val}");
            break;
        }
        is_visited[i] = true;
        is_visited[j] = true;
        dfs(pat.iter_mut().filter(|a| **a != v).collect(), es, max);
    }
    // println!("sum={sum}");
    if max < sum {
        max = sum;
    }
}
