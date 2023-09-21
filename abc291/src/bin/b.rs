use proconio::input;

fn main() {
    input! {
        n: usize,
        mut d: [u32;5*n],
    };
    d.sort();
    // println!("{:?}", d);
    for _ in 0..n {
        d.pop();
    }
    for _ in 0..n {
        d.remove(0);
    }
    // println!("{:?}", d);
    println!("{}", (d.iter().sum::<u32>() as f64) / 3.0 / (n as f64));
}
