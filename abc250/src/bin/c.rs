use proconio::input;

fn main() {
    input! {
        n: usize,
        num_q: usize,
        q: [usize;num_q],
    };
    let mut a = vec![0; n];
    let mut i_of = vec![0; n + 1];
    for i in 0..n {
        a[i] = i + 1;
        i_of[i + 1] = i;
    }
    for t in 0..num_q {
        let i = i_of[q[t]];
        let mut j = i_of[q[t]] + 1;
        if i == n - 1 {
            j = i_of[q[t]] - 1;
        }
        let ai = a[i];
        let aj = a[j];
        a[i] = aj;
        a[j] = ai;
        i_of[ai] = j;
        i_of[aj] = i;
    }
    println!(
        "{}",
        a.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
