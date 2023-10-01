use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [String;n],
        d: [String;m],
        p: [u32;m+1],
    };

    let mut sum = 0;
    for i in 0..n {
        if let Some(pos_d) = d.iter().position(|v| *v == c[i]) {
            sum += p[pos_d + 1];
        } else {
            sum += p[0];
        }
    }
    println!("{}", sum);
}
