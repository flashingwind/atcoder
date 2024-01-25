use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };
    let mut cnt = 0;
    while (n % 2) == 0 {
        // println!("{:b}", n);
        cnt += 1;
        n = n >> 1;
    }
    println!("{cnt}");
}
