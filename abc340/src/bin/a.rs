use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        d: usize,
    };
    let mut is_first = true;
    for i in (a..=b).step_by(d) {
        if is_first {
            print!("{}", i);
            is_first = false;
        } else {
            print!(" {}", i);
        }
    }
    println!();
    // println!("{}", (a..=d).step_by(b).map(|v| v.to_string()).join(" "));
}
