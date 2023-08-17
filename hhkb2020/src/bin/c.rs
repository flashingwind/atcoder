use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: [usize;n],
    };
    let m = 200_000;
    p.insert(0, 0);
    let mut v: Vec<_> = vec![false; m + 1];
    let mut min = 0;
    for i in 1..=n {
        v[p[i]] = true;
        if min == p[i] {
            for j in min + 1..=m {
                if !v[j] {
                    min = j;
                    break;
                }
            }
        }
        println!("{}", min);
    }
}
