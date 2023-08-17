use proconio::input;

fn main() {
    input! {
        n: usize,
        mut arr: [usize;n-1],
    };
    let mut cnt = vec![0usize; n + 1];
    for a in arr.iter() {
        cnt[*a] += 1;
    }
    for c in cnt.iter().skip(1) {
        println!("{}", c);
    }
}
