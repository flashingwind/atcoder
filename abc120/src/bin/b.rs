use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        k: u32,
    };
    let mut cnt = 0;
    for n in (1..=a.max(b)).rev() {
        if a % n == 0 && b % n == 0 {
            cnt += 1;
            if cnt == k {
                println!("{n}");
                return;
            }
        }
    }
}
