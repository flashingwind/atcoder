use proconio::input;

fn main() {
    input! {
        xy: f64,
        //a: [u32;n],
    };
    let x = xy as u64;
    let y = ((xy - x as f64) * 10.) as u64;
    // println!("{} {}", x, y);
    if 0 <= y && y <= 2 {
        println!("{}-", x);
    } else if 3 <= y && y <= 6 {
        println!("{}", x);
    } else if 7 <= y && y <= 9 {
        println!("{}+", x);
    }
}
