use proconio::input;

fn main() {
    input! {
        a: usize,
        b: u32,
        t: usize,
    };
    let mut cnt = 0;
    for round in 1..=21 {
        if round * a > t {
            break;
        }
        cnt += b;
    }
    println!("{}", cnt);
}
