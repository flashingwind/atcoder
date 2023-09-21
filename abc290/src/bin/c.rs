use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut arr: [usize;n],
    };
    arr.sort();
    arr.dedup();
    for x in 0..k {
        if arr.binary_search(&x).is_err() {
            println!("{}", x);
            return;
        }
    }
    println!("{}", k);
}
