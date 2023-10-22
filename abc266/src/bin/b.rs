use proconio::input;

fn main() {
    input! {
        n: i64,
    };
    let m = 998244353;
    let mut nn = n % m;
    if nn < 0 {
        nn += m;
    }
    println!("{}", nn);
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
