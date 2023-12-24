use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let lep = [
        0u64,
        1,
        11,
        111,
        1111,
        11111,
        111111,
        1111111,
        11111111,
        111111111,
        1111111111,
        11111111111,
        111111111111,
    ];
    let mut sums = vec![];
    for (rank, pat) in (1..=12usize).combinations_with_replacement(3).enumerate() {
        let mut sum = 0;
        for &len in pat.iter() {
            // print!("{}+", lep[len]);
            sum += lep[len];
        }
        sums.push(sum);
        // println!("rank={rank}:{}", sum);
    }
    sums.sort();
    println!("{}", sums[n - 1]);
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
