use proconio::input;

fn main() {
    let n = 9;
    input! {
        a: [u32;n],
        b: [u32;n-1],
    };
    println!("{}", a.iter().sum::<u32>() - b.iter().sum::<u32>() + 1);
}
