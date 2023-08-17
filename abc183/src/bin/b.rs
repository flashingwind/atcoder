use proconio::input;

fn main() {
    input! {
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64,
    };
    if sx == gx {
        print!("{}", sx);
    } else {
        let x = (sx * gy + gx * sy) / (sy + gy);
        print!("{}", x);
    }
}
