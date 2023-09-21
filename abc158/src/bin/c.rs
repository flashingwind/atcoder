use proconio::input;

fn main() {
    input! {
        a: f64,
        b: u64,
    };
    // let start = ((a - 1.) / 0.08).floor() as u64;
    // let end = ((a + 1.) / 0.08).floor() as u64;
    let start = (a / 0.09).floor() as u64;
    let end = (a / 0.07).floor() as u64;
    // println!("x={}..={}", start, end);
    for x in start..=end.max(100) {
        let bb = (x as f64 * 0.1).floor() as u64;
        if a != ((x as f64 * 0.08).floor()) {
            continue;
        }
        // println!("{}*0.1={}", x, bb);
        if b == bb {
            println!("{}", x);
            return;
        }
    }
    println!("-1");
}
