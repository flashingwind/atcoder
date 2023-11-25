use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize;m],
    };
    let mut cnt = vec![0; n + 1];
    let mut i_max = 0usize;
    for i in 0..m {
        cnt[a[i]] += 1;
        if cnt[i_max] < cnt[a[i]] {
            i_max = a[i];
        } else if cnt[i_max] == cnt[a[i]] {
            i_max = i_max.min(a[i]);
        }
        println!("{}", i_max);
    }
}
