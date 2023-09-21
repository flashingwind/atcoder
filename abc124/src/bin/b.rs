use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u32;n],
    };
    let mut cnt = 0;
    for (i, h_a) in h.iter().enumerate() {
        let mut is_ok = true;
        for h_b in h.iter().take(i + 1) {
            // println!("{i}:{h_b} < {h_a}");
            if *h_b > *h_a {
                // println!("false");
                is_ok = false;
                break;
            }
        }
        if is_ok {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
