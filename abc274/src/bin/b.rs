use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars;h],
    };
    let mut is_first = true;
    for i in 0..w {
        let mut cnt = 0;
        for j in 0..h {
            if c[j][i] == '#' {
                cnt += 1;
            }
        }
        if !is_first {
            print!(" ");
        } else {
            is_first = false;
        }
        print!("{}", cnt);
    }
    println!();
}
