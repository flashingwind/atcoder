use proconio::input;

fn main() {
    input! {
        y: usize,
    };
    for yy in y..=3002 {
        if yy % 4 == 2 {
            println!("{}", yy);
            break;
        }
    }
}
