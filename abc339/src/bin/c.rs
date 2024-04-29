use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64;n],
    };
    let mut sum = 0;
    let mut init = 0;
    for &ai in a.iter() {
        sum += ai;
        if sum < 0 {
            init -= sum;
            sum = 0;
        }
    }
    println!("{}", sum);
}
