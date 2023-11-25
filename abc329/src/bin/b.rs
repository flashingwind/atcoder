use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
    };
    let mut max = 0;
    for ai in a.iter() {
        max = max.max(*ai);
    }
    let mut max2 = 0;
    for i in 0..n {
        if a[i] != max {
            max2 = max2.max(a[i]);
        }
    }
    println!("{}", max2);
}
