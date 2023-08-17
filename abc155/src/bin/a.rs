use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        //d: [i32;n],
    };
    if (a == b && a != c) || (b != c && a == c) || (b == c && a != c) {
        println!("Yes");
    } else {
        println!("No");
    }
}
