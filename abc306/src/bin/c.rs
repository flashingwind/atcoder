use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 3 * n],
    };
    let mut cnt = vec![0usize; n + 1];
    let mut f = Vec::new();
    for (i, &ai) in a.iter().enumerate() {
        if cnt[ai] == 1 {
            f.push((i, ai));
        }
        cnt[ai] += 1;
    }
    f.sort();
    print!("{}", f[0].1);
    for (i, ai) in f.iter().skip(1) {
        print!(" {}", ai);
    }
    println!()
}
