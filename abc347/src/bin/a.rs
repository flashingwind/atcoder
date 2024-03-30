use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u32,
        a: [u32;n],
    };
    let mut is_first = true;
    for &aa in a.iter() {
        if aa % k == 0 {
            if !is_first {
                print!(" ");
            }
            print!("{}", aa / k);
            is_first = false;
        }
    }
}
