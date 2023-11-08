use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize;n],
    };
    a.sort();
    // let mut cnt = vec![0; n];
    // cnt[0] = 1;
    // for i in 1..n {
    //     cnt[i] = cnt[i - 1] + 1;
    // }
    // println!("{:?}", cnt);

    let mut max = 0;
    let mut left = 0;
    let mut right = left;
    let mut last_j = 0;
    for j in 0..n {
        if a[j] < a[0] + m {
            right = j + 1;
            last_j = j;
        } else {
            break;
        }
        // println!("{}:{left}~{}:{right} ={}", a[0], a[j], (right - left));
    }
    max = max.max(right - left);
    for i in 1..n {
        left = i;
        for j in last_j..n {
            if a[j] < a[i] + m {
                right = j + 1;
                last_j = j;
            } else {
                break;
            }
            // println!("{}:{left}~{}:{right} ={}", a[i], a[j], (right - left));
        }
        max = max.max(right - left);
    }
    println!("{max}");
}
