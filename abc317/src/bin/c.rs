use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut dist = vec![vec![0; n]; n];
    for j in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
            c: u64,
        }
        dist[a][b] = c;
        dist[b][a] = c;
    }
    let mut max = 0;
    for pat in (0..n).permutations(n) {
        let mut sum = 0;
        let mut last_node = pat[0];
        for &i in pat.iter().skip(1) {
            // println!(
            //     "  {last_node}->{i}: {sum}+{}={}",
            //     dist[last_node][i],
            //     sum + dist[last_node][i]
            // );
            if dist[last_node][i] == 0 {
                break;
            }
            sum += dist[last_node][i];
            last_node = i;
        }
        // println!("{:?}: {sum}", pat);
        if max < sum {
            max = sum;
        }
    }
    println!("{}", max);
}
