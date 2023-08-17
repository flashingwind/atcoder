use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h: [u32;n],
    };
    for i in (0..n - 1).rev() {
        if h[i] <= h[i + 1] {
        } else {
            // println!("{:?}", h);
            h[i] -= 1;
            if h[i] <= h[i + 1] {
            } else {
                // println!("{:?}", h);
                // println!("h{}:{} > h{}:{}", i, h[i], i + 1, h[i + 1]);
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
