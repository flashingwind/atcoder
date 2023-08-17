use proconio::input;

fn main() {
    input! {
        k: u32,
    };
    let mut gcd_sum = 0;
    for a in 1..=k {
        for b in 1..=k {
            for c in 1..=k {
                gcd_sum += gcd(gcd(a, b), c);
            }
        }
    }
    println!("{}", gcd_sum);
}
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
