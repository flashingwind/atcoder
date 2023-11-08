use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32;n],
    };
    let mut min = 32; //log_2(10^9)
    for i in 0..n {
        let mut cnt = 0;
        while a[i] % 2 == 0 && cnt <= min {
            a[i] /= 2;
            cnt += 1;
        }
        min = min.min(cnt);
        // println!("{i}:{cnt} min={min}");
    }
    println!("{min}");
}
