use proconio::input;

fn main() {
    input! {
        n: usize,
        mut c_arr: [usize;n],
    };
    c_arr.sort();
    let mut cnt = 1;
    let max = 10usize.pow(9) + 7;
    for (i, c) in c_arr.iter().enumerate() {
        cnt *= *c - i;
        cnt %= max;
    }
    println!("{}", cnt % max);
}
