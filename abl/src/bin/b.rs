use proconio::input;

fn main() {
    input! {
        a: (u64,u64),
        b: (u64,u64),
    };
    if a.1 >= b.0 && b.1 >= a.0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
