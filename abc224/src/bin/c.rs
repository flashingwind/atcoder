use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64,i64);n],
    };
    let x = xy.iter().map(|a| a.0).collect::<Vec<i64>>();
    let y = xy.iter().map(|a| a.1).collect::<Vec<i64>>();
    let mut cnt = 0;
    for i1 in 0..n {
        for i2 in i1 + 1..n {
            for i3 in i2 + 1..n {
                // println!(
                //     "{i1},{i2},{i3}={}",
                //     (x[i2] - x[i1]) * (y[i3] - y[i1]) - (x[i3] - x[i1]) * (y[i2] - y[i1])
                // );
                if (x[i2] - x[i1]) * (y[i3] - y[i1]) - (x[i3] - x[i1]) * (y[i2] - y[i1]) != 0 {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt);
}
