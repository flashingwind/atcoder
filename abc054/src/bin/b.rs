use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars;n],
        b: [Chars;m],
    };
    for i in 0..=n - m {
        for j in 0..=n - m {
            let mut is_match = true;
            for i2 in 0..m {
                for j2 in 0..m {
                    // println!("{},{}: {i2},{j2}", i + i2, j + j2);
                    if a[i + i2][j + j2] != b[i2][j2] {
                        is_match = false;
                        // println!("  break");
                        break;
                    }
                }
                if !is_match {
                    break;
                }
            }
            // println!();
            if is_match {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
