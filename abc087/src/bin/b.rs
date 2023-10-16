use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        x: u32,
    };
    let mut cnt = 0;
    for n500 in 0..=a {
        for n100 in 0..=b {
            for n50 in 0..=c {
                if n500 * 500 + n100 * 100 + n50 * 50 == x {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt);
}
