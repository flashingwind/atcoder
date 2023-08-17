use proconio::input;

fn main() {
    let l = 64;
    input! {
        d: [u64;l],
    };
    let mut sum = 0;
    for i in 0..l {
        sum += d[i] * 2_u64.pow(i as u32);
        // println!("{}", d[i] * 2_u64.pow(i as u32));
    }
    println!("{}", sum);
}
