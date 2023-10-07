use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n],
    };
    let mut sum = 0;
    for &ai in a.iter() {
        if 10 < ai {
            sum += ai - 10;
        }
    }
    println!("{}", sum);
}
