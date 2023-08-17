use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u32;w];h],
    };
    for j in 0..w {
        print!("{}", a[0][j]);
        for i in 1..h {
            print!(" {}", a[i][j]);
        }
        println!();
    }
}
