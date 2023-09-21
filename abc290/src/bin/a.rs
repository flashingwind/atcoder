use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u32;n],
        b: [usize;m],
    };
    a.insert(0, 0);
    let mut sum = 0;
    for prob in b.iter() {
        sum += a[*prob];
    }
    println!("{}", sum);
}
