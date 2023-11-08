use proconio::input;

fn main() {
    input! {
        a: i64,
        b_plus: i64,
        r_plus: i64,
        d: i64,
    };
    let mut blue = a;
    let mut red = 0;
    if r_plus * d <= b_plus {
        println!("-1");
        return;
    }
    for t in 1..=a {
        blue += b_plus;
        red += r_plus;
        if blue <= red * d {
            println!("{}", t);
            return;
        }
    }
    println!("-1");
}
