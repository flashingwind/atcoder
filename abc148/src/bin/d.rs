use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
    };
    a.insert(0, 0);
    let mut i = 1;
    let mut cnt = 0;
    for j in 1..=n {
        while i <= n && j != a[i] {
            cnt += 1;
            i += 1;
        }
        i += 1;
        if n <= i {
            break;
        }
    }
    if n == cnt {
        println!("-1");
    } else {
        println!("{}", cnt);
    }
}
