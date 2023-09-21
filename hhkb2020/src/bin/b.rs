use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars;h],
    };
    let mut cnt = 0;
    for y in 0..h {
        for x in 0..w {
            if s[y][x] == '.' {
                if y + 1 < h && s[y + 1][x] == '.' {
                    cnt += 1;
                }
                if x + 1 < w && s[y][x + 1] == '.' {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt);
}
