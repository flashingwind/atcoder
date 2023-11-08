use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n],
    };
    let mut p = 0;
    let mut b = vec![false; 5];
    for i in 0..n {
        b[0] = true;
        for x in (0..=4).rev() {
            if b[x] {
                if 3 < x + a[i] {
                    p += 1;
                    b[x] = false;
                } else {
                    b[x] = false;
                    b[x + a[i]] = true;
                }
            }
        }
    }
    println!("{p}");
}
