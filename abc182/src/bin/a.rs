use proconio::input;

fn main() {
    input! {
        followers: u32,
        followings: u32,
    };
    println!("{}", (2 * followers + 100) - followings);
}
