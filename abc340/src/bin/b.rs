use proconio::input;

fn main() {
    input! {
        num_q: usize,
    };
    let mut a = vec![];
    for t in 0..num_q {
        input! {
            typ: usize,
        };
        if typ == 1 {
            input! {
                x: u64,
            };
            a.push(x);
        } else {
            input! {
                k: usize,
            };
            println!("{}", a[a.len() - k]);
        }
    }
}
