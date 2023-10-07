use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };
    let mut or = a | b;
    // println!("{:b}", or);
    let mut sum = 0;
    let mut n = 1;
    while or != 0 {
        if or % 2 == 1 {
            sum += n;
            // println!(" +{}", n);
        }
        n *= 2;
        or /= 2;
    }
    println!("{}", sum);
}
