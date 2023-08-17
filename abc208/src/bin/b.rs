use proconio::input;

fn main() {
    input! {
        mut p: u32,
    };
    let mut cnt = 0;
    for n in (1..=10).rev() {
        let coin = fact(n);
        if coin <= p {
            cnt += p / coin;
            p %= coin;
        }
        if p == 0 {
            break;
        }
    }
    println!("{}", cnt);
}

fn fact(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        fact(n - 1) * n
    }
}
