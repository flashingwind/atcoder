use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [u32;n],
    };
    let max = *p.iter().max().unwrap();
    let cnt_max = p.iter().filter(|v| **v == max).count();
    if 1 == cnt_max && p[0] == max {
        println!("{}", max - p[0]);
    } else {
        println!("{}", max - p[0] + 1);
    }
}
