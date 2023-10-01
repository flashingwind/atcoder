use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize;m],
    };
    a.reverse();
    let mut d_hanabi = a.pop().unwrap();
    for d in 1..=n {
        if d_hanabi < d {
            if let Some(tmp) = a.pop() {
                d_hanabi = tmp;
            } else {
                return;
            }
        }
        println!("{}", d_hanabi - d);
    }
}
