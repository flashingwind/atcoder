use proconio::input;

fn main() {
    input! {
        ta1: u32,
        ta2: u32,
        ao1: u32,
        ao2: u32,
    };
    if ta1 < ao1 || ta1 == ao1 && ta2 <= ao2 {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
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
