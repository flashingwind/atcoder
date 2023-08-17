use proconio::input;

fn main() {
    input! {
        x: u32,
    };
    if 400 <= x && x <= 599 {
        println!("8");
    } else if 600 <= x && x <= 799 {
        println!("7");
    } else if 800 <= x && x <= 999 {
        println!("6");
    } else if 1000 <= x && x <= 1199 {
        println!("5");
    } else if 1200 <= x && x <= 1399 {
        println!("4");
    } else if 1400 <= x && x <= 1599 {
        println!("3");
    } else if 1600 <= x && x <= 1799 {
        println!("2");
    } else if 1800 <= x && x <= 1999 {
        println!("1");
    }
}
