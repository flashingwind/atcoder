use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [u128;n],
    };
    a.insert(0, 0);
    // let mut last_prod = a[1];
    // for (j, _) in a.iter().enumerate().skip(1).take(k) {
    //     //i=k-1, 1..=k
    //     last_prod *= a[j];
    // }
    for i in 2..=n - k + 1 {
        // let mut prod = last_prod;
        // prod /= a[i - 1];
        // prod *= a[i + k - 1];
        // print!("i={i}: prod{i}-{}", i + k - 1);
        // if last_prod < prod {
        if a[i - 1] < a[i + k - 1] {
            println!("Yes");
        } else {
            println!("No");
        }
        // last_prod = prod;
    }
}
