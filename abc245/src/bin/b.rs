use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32;n],
    };
    a = a.iter().map(|v| *v).sorted().unique().collect();
    if 0 < a[0] {
        println!("0");
        return;
    }
    for i in 1..a.len() {
        if 1 < a[i] - a[i - 1] {
            println!("{}", a[i - 1] + 1);
            return;
        }
    }
    println!("{}", *a.last().unwrap() + 1)
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
