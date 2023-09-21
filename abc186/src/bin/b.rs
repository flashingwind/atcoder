use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize;w];h],
    };
    let mut min = std::usize::MAX;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] < min {
                min = a[i][j];
            }
        }
    }
    let mut cost = 0;
    for i in 0..h {
        for j in 0..w {
            cost += a[i][j] - min;
        }
    }
    println!("{}", cost);
}
