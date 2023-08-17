use proconio::input;

struct person {
    won: u64,
    id: usize,
    rank: usize,
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[char;m];2*n],
    };
    let mut rank = vec![(0usize, 0usize); 2 * n];
    let mut cnt_win = vec![0usize; 2 * n];
    for t in 0..m {
        for k_raw in 0..n {
            letk = rank[k_raw].0;
            // 2n-2 vs 2n-1
            if is_left_won(a[2 * k][t], a[2 * k + 1][t]) {
                cnt_win[2 * k] += 1;
            } else if is_left_won(a[2 * k + 1][t], a[2 * k][t]) {
                cnt_win[2 * k + 1] += 1;
            }
        }
    }
}

fn is_left_won(c1: char, c2: char) -> bool {
    if c1 == 'G' && c2 == 'C' || c1 == 'C' && c2 == 'P' || c1 == 'P' && c2 == 'G' {
        return true;
    }
    return false;
}
