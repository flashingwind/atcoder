use proconio::input;

fn main() {
    input! {
        mut y: i64,
        mut x: i64,
    };
    y -= 8;
    x -= 8;

    let max = x.abs().max(y.abs());
    println!("{}", if max % 2 == 0 { "white" } else { "black" });
    // for dy in -7..=7i64 {
    //     for dx in -7..=7i64 {
    //         let max = dx.abs().max(dy.abs());
    //         print!("{}", if max % 2 == 0 { "■" } else { "□" });
    //     }
    //     println!();
    // }
}
