use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
        b: [u32;n],
    };
    let min = a.iter().max().unwrap();
    let max = b.iter().min().unwrap();
    if min <= max {
        // println!("{}-{}", max, min);
        println!("{}", max - min + 1);
    } else {
        println!("0");
    }
}
