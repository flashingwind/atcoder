use proconio::input;

fn main() {
    input! {
        r: u32,
        mut g: u32,
        mut b: u32,
        k: u32,
    };
    // r<g<b
    if r < g && g < b {
        println!("Yes");
        return;
    }
    for _ in 0..k {
        if r >= g {
            g *= 2;
        } else if g >= b {
            b *= 2;
        }
        if r < g && g < b {
            println!("Yes");
            return;
        }
        // println!("{},{},{}", r, g, b);
    }
    println!("No");
}
//
