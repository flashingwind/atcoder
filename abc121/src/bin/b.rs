use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: i32,
        b: [i32;m],
        a_mat: [[i32;m];n],
    };
    let mut cnt = 0;
    for a in a_mat.iter() {
        let mut sum = c;
        for i in 0..m {
            sum += a[i] * b[i];
        }
        if 0 < sum {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
