use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u32,
        aa: [u32;n],
        bb: [u32;n],
    };
    // 0=a, 1=b
    let mut a = vec![];
    a.push(aa);
    a.push(bb);
    let mut is_ok = vec![vec![false; n]; 2];
    is_ok[0][0] = true;
    is_ok[1][0] = true;
    for i in 1..n {
        for last_ab in 0..=1 {
            if is_ok[last_ab][i - 1] {
                for ab in 0..=1 {
                    let last_x = a[last_ab][i - 1];
                    let x = a[ab][i];
                    if x.abs_diff(last_x) <= k {
                        is_ok[ab][i] = true;
                    }
                }
            }
        }
    }
    // println!("A:{:?}", is_ok[0]);
    // println!("B:{:?}", is_ok[1]);
    if is_ok[0][n - 1] || is_ok[1][n - 1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
