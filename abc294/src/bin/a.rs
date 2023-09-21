use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
    };
    let mut is_first = true;
    for ai in a.iter() {
        if *ai % 2 == 0 {
            if !is_first {
                print!(" ");
            }
            print!("{}", *ai);
            is_first = false;
        }
    }
    println!();
}
