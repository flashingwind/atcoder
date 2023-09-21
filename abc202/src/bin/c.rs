use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n],
        b: [usize;n],
        c: [usize;n],
    };
    let mut cnt = 0;
    let mut cnt_b: Vec<usize> = vec![0; n + 1];
    for (j, _) in b.iter().enumerate() {
        cnt_b[b[c[j] - 1]] += 1;
    }
    // println!("{:?}", cnt_b);
    for (i, _) in a.iter().enumerate() {
        cnt += cnt_b[a[i]];
    }
    println!("{}", cnt);
}
