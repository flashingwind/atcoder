use proconio::input;

fn main() {
    input! {
        n: u32,
    };
    for cnt_a in 0..25 {
        if n < cnt_a * 4 {
            break;
        }
        if (n - cnt_a * 4) % 7 == 0 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
