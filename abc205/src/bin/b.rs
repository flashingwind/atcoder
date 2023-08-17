use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
    };
    a.sort();
    // println!("{:?}", a);
    for k in 1..=n {
        let i = k - 1;
        if a[i] != k {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
