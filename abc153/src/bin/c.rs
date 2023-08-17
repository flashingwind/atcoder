use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut h: [u64;n],
    };
    h.sort();
    for hp in h.iter_mut().rev() {
        if k != 0 {
            // println!("hp={hp}");
            *hp = 0;
            k -= 1;
        } else {
            break;
        }
    }
    let sum: u64 = h.iter().sum();
    println!("{}", sum);
}
