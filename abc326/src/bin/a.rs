use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    };
    if x <= y {
        if (y - x) <= 2 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if (x - y) <= 3 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
