use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
    };
    a.sort();
    let mut cnt = vec![0; 100001];
    for i in 0..n {
        cnt[a[i]] += 1;
    }
    let mut max = 0;
    for (j, _) in cnt.iter().enumerate().skip(1).take(99999) {
        if max < cnt[j - 1] + cnt[j] + cnt[j + 1] {
            max = cnt[j - 1] + cnt[j] + cnt[j + 1];
        }
    }
    println!("{max}");
}
