use proconio::input;

fn main() {
    input! {
        l: [u32;2],
        r: [u32;2],
    };
    if l.iter().sum::<u32>() > r.iter().sum::<u32>() {
        println!("Left");
    } else if l.iter().sum::<u32>() < r.iter().sum::<u32>() {
        println!("Right");
    } else {
        println!("Balanced");
    }
}
