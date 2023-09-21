use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        h_sel: usize,
        w_sel: usize,
    };
    println!("{}", (h - h_sel) * (w - w_sel));
}
