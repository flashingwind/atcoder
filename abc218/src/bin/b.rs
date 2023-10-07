use proconio::input;

fn main() {
    input! {
        a: [u8;26],
    };
    for i in 0..26 {
        print!("{}", (a[i] - 1 + 'a' as u8) as char);
    }
    println!();
}
