use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
    };
    let mut cnt_same = 0usize;
    for i in 0..n {
        a[i] -= 1;
        if a[i] == i {
            cnt_same += 1;
        }
    }
    let mut ans = cnt_same * (cnt_same - 1) / 2;
    for i in 0..n {
        if a[i] > i && a[a[i]] == i {
            ans += 1;
        }
    }
    println!("{}", ans);
}
