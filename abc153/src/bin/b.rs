use proconio::input;

fn main() {
    input! {
        mut h: u32,
        n: usize,
        arr: [u32; n],
    };
    for a in arr.iter() {
        if *a < h {
            h -= *a;
        } else {
            h = 0;
        }
        if h == 0 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
