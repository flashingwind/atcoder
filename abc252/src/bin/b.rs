use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u32;n],
        b: [usize;k],
    };
    let max_a = *a.iter().max().unwrap();
    let aa: Vec<_> = a.iter().enumerate().filter(|&(i, &v)| v == max_a).collect();

    for &(i, _) in aa.iter() {
        if b.contains(&(i + 1)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
