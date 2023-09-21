use proconio::input;

fn main() {
    input! {
        mut n: usize,
        mut a: [usize;n],
        mut b: [u32;n],
        mut c: [u32;n-1],
    };
    a.insert(0, 0);
    b.insert(0, 0);
    c.insert(0, 0);
    let mut sum = 0;
    let mut before = 0;
    for &i in a.iter().skip(1) {
        sum += b[i];
        if before != 0 && before + 1 == i {
            sum += c[before];
        }
        before = i;
    }
    println!("{}", sum);
}
