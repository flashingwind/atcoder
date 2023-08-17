use proconio::input;

fn main() {
    input! {
        age: u64,
        byen: u64,
    };
    if age <= 5 {
        println!("0");
    } else if 13 <= age {
        println!("{}", byen);
    } else {
        println!("{}", byen / 2);
    }
}
