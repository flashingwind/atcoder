use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    };
    if (a <= 0 && 0 <= b) || (b <= 0 && 0 <= a) {
        // ゼロを含む
        println!("Zero");
    } else if 0 < a && 0 < b {
        // 両方、正
        println!("Positive");
    } else if a < 0 && b < 0 {
        // 両方、負
        if (b - a + 1) % 2 == 0 {
            println!("Positive");
        } else {
            println!("Negative");
        }
    }
}
