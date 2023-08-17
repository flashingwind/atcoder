use proconio::input;

fn main() {
    input! {
        abc: u32,
    };
    let mut sum = abc;
    let c = sum % 10;
    sum /= 10;
    let b = sum % 10;
    sum /= 10;
    let a = sum % 10;
    // println!("{a} {b} {c}");
    // println!(
    //     "{} {} {}",
    //     abc,
    //     (b * 100 + c * 10 + a),
    //     (c * 100 + a * 10 + b)
    // );
    println!("{}", abc + (b * 100 + c * 10 + a) + (c * 100 + a * 10 + b));
}
