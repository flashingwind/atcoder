use proconio::input;

fn main() {
    input! {
        mut g: i32,
        mut w: i32,
        mut h: i32,
    };
    if w < 0 {
        g *= -1;
        w *= -1;
        h *= -1;
    }

    if g < w {
        println!("{}", g.abs());
    } else {
        if w < h {
            println!("-1");
        } else {
            println!("{}", h.abs() + (g - h).abs());
        }
    }
}
