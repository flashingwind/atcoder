use proconio::input;

fn main() {
    input! {
        s: [String;2],
        mut n: [usize;2],
        u: String
    };
    if s[0] == u {
        n[0] -= 1;
    } else {
        n[1] -= 1;
    }
    println!("{} {}", n[0], n[1]);
}
