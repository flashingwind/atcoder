use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [(String,u32);n],
    };
    let mut min_i = 0;
    let mut min = u32::MAX;
    for j in 0..n {
        if p[j].1 < min {
            min_i = j;
            min = p[j].1;
        }
    }
    for jj in min_i..min_i + n {
        let j = jj % n;
        // println!("{jj}:{j}:{}", p[j].0);
        println!("{}", p[j].0);
    }
}
