// use std::collections::HashSet;
use num::integer::gcd;
use proconio::input;
fn main() {
    input! {
        a: u64,
        b: u64,
    };
    // let f_a = factors(a);
    // let f_b = factors(b);
    // let uniq: HashSet<_> = f_a.iter().chain(f_b.iter()).collect();
    // println!("{a}={:?}", f_a.iter().collect::<Vec<_>>());
    // println!("{b}={:?}", f_b.iter().collect::<Vec<_>>());
    // println!("{:?}", uniq);
    // let mut common = 1;
    // for x in uniq.iter() {
    //     common *= *x;
    // }
    println!("{}", a * b / gcd(a, b));
}

// fn factors(mut n: u64) -> Vec<u64> {
//     let mut out = vec![];
//     for i in 2..(n + 1) {
//         while n % i == 0 {
//             out.push(i);
//             n /= i;
//         }
//         if n == 1 {
//             break;
//         }
//     }
//     out
// }
