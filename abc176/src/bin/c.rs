use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64;n],
    };
    let mut sum = 0;
    let mut now = a[0];
    for i in 1..n {
        if a[i] < now {
            sum += now - a[i];
            // println!(
            //     "a{}:{} - a{}:{}={}",
            //     i - 1,
            //     i,
            //     a[i - 1],
            //     a[i],
            //     a[i - 1] - a[i]
            // );
        } else if now < a[i] {
            now = a[i];
        }
    }
    println!("{}", sum);
}
