use proconio::input;

fn main() {
    input! {
        n: usize,//last
        m: usize,//now
        x: usize,//stop
        t: usize,//last,stop
        d: usize,
    };
    if x <= m {
        println!("{}", t);
    } else {
        println!("{}", t - (x - m) * d);
    }
}
