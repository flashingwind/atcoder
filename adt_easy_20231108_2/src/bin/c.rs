use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
    };
    a.sort();
    for i in 0..=2000 {
        if !a.contains(&i) {
            println!("{}", i);
            return;
        }
    }
}
