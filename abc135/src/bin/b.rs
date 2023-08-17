use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize;n],
    };
    let mut cnt = 0;
    for i in 0..n {
        if p[i] != i + 1 {
            cnt += 1;
        }
        if 2 < cnt {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
