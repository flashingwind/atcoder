use proconio::input;

fn main() {
    input! {
        k: usize,
        g: u32,
        m: u32,
    };
    let mut gg = 0;
    let mut mm = 0;
    for _ in 0..k {
        if gg == g {
            gg = 0;
        } else if mm == 0 {
            mm = m;
        } else {
            if mm <= g - gg {
                gg += mm;
                mm = 0;
            } else {
                mm -= g - gg;
                gg = g;
            }
        }
    }
    println!("{} {}", gg, mm);
}
