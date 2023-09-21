use proconio::input;

fn main() {
    input! {
        n: u32,
    };
    for i in 1..=n {
        let s = i * (i + 1);
        // println!("s={s}");
        if n * 2 <= s {
            println!("{}", i);
            break;
        }
    }
}
