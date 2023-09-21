use proconio::input;

fn main() {
    input! {
        n: usize,
        mut c: [u32;n],
    };
    let mut a = 0;
    let mut b = 0;
    c.sort();
    for (i, pt) in c.iter().rev().enumerate() {
        if i % 2 == 0 {
            a += *pt;
        } else {
            b += *pt;
        }
    }
    println!("{}", a - b);
}
