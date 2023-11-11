use proconio::input;

fn main() {
    input! {
        ss: String,
    };
    let mut s = ss.clone();
    let mut len = usize::MAX;
    while len != s.len() {
        len = s.len();
        s = s.replace("ABC", "").clone();
    }
    if s.len() != 0 {
        println!("{}", s);
    }
}
