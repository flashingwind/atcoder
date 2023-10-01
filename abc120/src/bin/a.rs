use proconio::input;

fn main() {
    input! {
        p: i32,
        sum: i32,
        c: i32,
    };
    if sum <= c * p {
        println!("{}", sum / p);
    } else {
        println!("{}", c);
    }
}
