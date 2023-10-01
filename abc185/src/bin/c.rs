use proconio::input;

fn main() {
    input! {
        l: u128,
    };

    // nCr n=l-1, r=11
    // n*(n-1)*……*(n-r+1)!

    let mut cnt = 1;
    for i in (l - 1) - 10..=(l - 1) {
        cnt *= i;
    }
    // println!("{}", cnt);
    let mut div = 1;
    for i in 2..=11 {
        div *= i;
    }
    println!("{}", cnt / div);
}
