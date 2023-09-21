use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize;n],
    };
    a.sort();
    a.reverse();
    let sum: usize = a.iter().sum();
    // println!("{:?} sum/4m={}", a, sum / (4 * m));
    for aa in a.iter().take(m) {
        if *aa * (4 * m) < sum {
            //*aa < (sum / (4 * m))
            println!("No");
            // println!("aa={}", aa);
            return;
        }
    }
    println!("Yes");
}
