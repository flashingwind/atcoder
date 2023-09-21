use proconio::input;

fn main() {
    input! {
        n: usize,
        t: u32,
        c: [u32;n],
        r: [u32;n],
    };
    let mut max_i = std::usize::MAX;
    for i in 0..n {
        if c[i] == t {
            if max_i == std::usize::MAX || r[max_i] < r[i] {
                max_i = i;
            }
        }
    }
    if max_i == std::usize::MAX {
        max_i = 0;
        for i in 0..n {
            if c[i] == c[0] {
                if r[max_i] < r[i] {
                    max_i = i;
                }
            }
        }
    }
    println!("{}", max_i + 1);
}
