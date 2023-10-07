use proconio::input;

fn main() {
    input! {
        n: usize,
        s: i32,
        t: i32,
        mut w: i32,
        a: [i32;n-1],
    };
    let mut cnt = 0;
    if s <= w && w <= t {
        cnt += 1;
    }
    for &ai in a.iter() {
        w += ai;
        if s <= w && w <= t {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
