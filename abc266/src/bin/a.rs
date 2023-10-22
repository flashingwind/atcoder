use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    println!("{}", s[s.len() / 2]);
}
// fn spider(){
// let d = vec![
//     (-1_i64, -1_i64),
//     (0, -1),
//     (1, -1),
//     (-1, 0),
//     (1, 0),
//     (-1, 1),
//     (-1, 1),
//     (0, 1),
//     (1, 1),
// ];
// }
