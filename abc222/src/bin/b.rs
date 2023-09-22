use proconio::input;

fn main() {
    input! {
        n: usize,
        p: u32,
        a: [u32;n],
    }
    let mut cnt = 0;
    for aa in a.iter() {
        if *aa < p {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
