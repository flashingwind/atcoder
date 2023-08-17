use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [[u32;n];n],
        mut b: [[u32;n];n],
    };
    for _ in 0..4 {
        let mut is_ok = true;
        // check
        for i in 0..n {
            for j in 0..n {
                if a[i][j] == 1 {
                    if b[i][j] == 1 {
                    } else {
                        is_ok = false;
                        break;
                    }
                }
            }
        }
        if is_ok {
            println!("Yes");
            return;
        }
        // trans
        let mut aa = vec![vec![0u32; n]; n];
        for i in 0..n {
            for j in 0..n {
                // println!("{} {}", n - j - 1, i);
                aa[i][j] = a[n - j - 1][i];
            }
        }
        a = aa.to_owned();
    }
    println!("No");
}
