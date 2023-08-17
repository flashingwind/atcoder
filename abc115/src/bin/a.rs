use proconio::input;

fn main() {
    input! {
        mut d: usize,
    };
    if 22 <= d && d <= 25 {
        print!("Christmas");
        while d < 25 {
            print!(" Eve");
            d += 1;
        }
        println!();
    }
}
