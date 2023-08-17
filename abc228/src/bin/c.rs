use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [[u32;3];n],
    };
    if n <= k {
        println!("{}", "Yes\n".repeat(n));
        return;
    }
    let th: u32 = p
        .iter()
        .map(|x| x.iter().sum::<u32>())
        .sorted()
        .rev()
        .take(k)
        .last()
        .unwrap();
    for pp in p.iter() {
        if th <= pp.iter().sum::<u32>() + 300 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
