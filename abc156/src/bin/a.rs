use proconio::input;

fn main() {
    input! {
        k: i32,
        pub_r: i32,
    };
    if k < 10 {
        // public = inner -100*(100-k)
        println!("{}", pub_r + 100 * (10 - k));
    } else {
        println!("{}", pub_r);
    }
}
