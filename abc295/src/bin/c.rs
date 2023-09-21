use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32;n],
    };
    a.sort();
    let mut cnt = 0;
    let mut is_paired = false;
    for i in 0..n - 1 {
        if !is_paired && a[i] == a[i + 1] {
            cnt += 1;
            is_paired = true;
        } else {
            is_paired = false;
        }
    }
    println!("{}", cnt);
}
