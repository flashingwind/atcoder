use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [u32;n],
    };
    let mut cnt = 0;
    for i in 1..n - 1 {
        if p[i - 1] < p[i] && p[i] < p[i + 1] || p[i + 1] < p[i] && p[i] < p[i - 1] {
            // println!("{i}: pi-1={} pi={} pi+1={}", p[i - 1], p[i], p[i + 1]);
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
