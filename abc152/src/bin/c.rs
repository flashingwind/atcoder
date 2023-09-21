use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: [usize;n],
    };
    k.insert(0, 0);
    let mut min_k_below_i = k[1];
    let mut cnt = 0;
    for i in 1..=n {
        if k[i] <= min_k_below_i {
            min_k_below_i = k[i];
            // println!("{} == {}", k[i], min_k_below_i);
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
