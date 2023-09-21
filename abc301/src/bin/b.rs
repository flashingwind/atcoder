use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32;n],
    };

    let mut i = 1;
    print!("{}", a[0]);
    while i < a.len() {
        if a[i - 1] + 1 < a[i] {
            for j in a[i - 1] + 1..a[i] {
                print!(" {}", j);
            }
        } else if a[i - 1] > a[i] + 1 {
            for j in (a[i] + 1..a[i - 1]).rev() {
                print!(" {}", j);
            }
        }
        print!(" {}", a[i]);
        i += 1;
    }
    println!();
}
