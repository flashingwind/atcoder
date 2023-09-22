use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
                n: u64,
                a: u64,
                b: u64,
                c: u64,
        }
        let mut cnt = 0;
        for x in 0..=n {
            for y in 0..=n {
                if (a * x + b * y) % c == 0 {
                    cnt += 1;
                }
            }
        }
        println!("{}", cnt);
    }
}
