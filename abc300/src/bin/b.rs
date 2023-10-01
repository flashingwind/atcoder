use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars;h],
        b: [Chars;h],
    };
    for s in 0..h {
        for t in 0..w {
            let mut is_ok = true;
            for i in 0..h {
                for j in 0..w {
                    if a[(i + s) % h][(j + t) % w] != b[i][j] {
                        is_ok = false;
                        break;
                    }
                }
                if !is_ok {
                    break;
                }
            }
            if is_ok {
                // println!("{} {}", s, t);
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
