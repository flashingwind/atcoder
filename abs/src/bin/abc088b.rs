use proconio::input;
fn main() {
    input! {
        n: usize,
    };

    input! {
        mut a: [i32; n],
    };

    let mut p_a = 0;
    let mut p_b = 0;
    for i in 0..n {
        let mut i_max = 0;
        for ii in 0..n {
            if a[i_max] < a[ii] {
                i_max = ii;
            }
        }
        if i % 2 == 0 {
            p_a += a[i_max];
        } else {
            p_b += a[i_max];
        }
        a[i_max] = 0;
    }
    println!("{}", p_a - p_b);
}
