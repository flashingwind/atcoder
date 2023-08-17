use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        //d: [i32;n],
    };
    if b == c && c == d && (n - b * 3 <= a || n < a - b * 4) {
        // XYYX*b
        // (X)XYY(X…)XYYX(X…)XYYX(X)
        println!("Yes");
    }
}
