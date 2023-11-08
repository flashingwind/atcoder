use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        y: u64,
    };
    let mut red = vec![0u64; n + 1];
    let mut blu = vec![0u64; n + 1];
    red[n] = 1;
    // println!("red:{:?}", red);
    // println!("blu:{:?}", blu);
    for i in (2..=n).rev() {
        let r = red[i];
        red[i] = 0;
        red[i - 1] += r;
        blu[i] += r * x;
        let b = blu[i];
        blu[i] = 0;
        red[i - 1] += b;
        blu[i - 1] += b * y;
        // println!("red:{:?}", red);
        // println!("blu:{:?}", blu);
    }
    println!("{}", blu[1]);
}
