use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    if n % 10 == 3 {
        // N の 1 の位が 3 のとき bon
        println!("bon");
    } else if n % 10 == 0 || n % 10 == 1 || n % 10 == 6 || n % 10 == 8 {
        // N の 1 の位が 0,1,6,8 のとき pon
        println!("pon");
    } else {
        // N の 1 の位が 2,4,5,7,9 のとき hon
        println!("hon");
    }
}
