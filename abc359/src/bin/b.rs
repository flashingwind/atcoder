use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n*2],
    };
    let mut cnt = 0;
    for i in 0..=(n * 2) - 3 {
        // println!("{i}:{}{}{}", a[i], a[i + 1], a[i + 2]);
        if a[i] == a[i + 2] {
            cnt += 1;
        }
    }
    println!("{cnt}");
}
