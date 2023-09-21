use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: u32,
        m: [u32;n],
    };
    let mut cnt = 0;
    let mut min_mi = std::u32::MAX;
    for mi in m.iter() {
        x -= *mi;
        cnt += 1;
        if *mi < min_mi {
            min_mi = *mi;
        }
    }
    while min_mi <= x {
        x -= min_mi;
        cnt += 1;
    }
    println!("{}", cnt);
}
