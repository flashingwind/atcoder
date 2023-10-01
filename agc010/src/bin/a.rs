use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64;n],
    };
    let mut odd_cnt = 0usize;
    for i in 0..n {
        if a[i] % 2 == 1 {
            odd_cnt += 1;
        }
    }
    if odd_cnt % 2 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
