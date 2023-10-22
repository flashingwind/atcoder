use proconio::input;

fn main() {
    input! {
        l: u32,
        r: u32,
        d: u32,
    };
    let mut cnt = 0;
    for n in l..=r {
        if n % d == 0 {
            cnt += 1;
        }
    }
    println!("{cnt}");
}
