use proconio::input;

fn main() {
    input! {
        a: u128,
        b: u128,
        c: u128,
        d: u128,
        e: u128,
        f: u128,
    };
    let mul1 = mul(mul(a, b), c);
    let mul2 = mul(mul(d, e), f);
    println!("{}", (mul1 + 998244353 - mul2) % 998244353);
}
fn mul(a: u128, b: u128) -> u128 {
    return ((a % 998244353) * (b % 998244353)) % 998244353;
}
