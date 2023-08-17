use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: [usize;n-1],
    };
    p.insert(0, 0);
    p.insert(0, 0);
    let mut i = n;
    let mut cnt = 0;
    while 1 < i {
        i = p[i];
        cnt += 1;
        // println!("pi={} i={i}", p[i]);
    }
    println!("{}", cnt);
}
