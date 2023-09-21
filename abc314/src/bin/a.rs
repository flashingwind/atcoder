use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let s = String::from("3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679");
    for i in 0..n + 2 {
        print!("{}", s.chars().nth(i).unwrap());
    }
    println!();
}
