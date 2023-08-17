use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u32,
        xs: [u32;n],
    };
    if 16 <= n {
        return;
    }
    let mut cnt = 0;
    for pat in 0..=(1usize << n) - 1 {
        let mut sum = 0;
        let mut card_cnt = 0;
        for (i, x) in xs.iter().enumerate() {
            if (pat & (1 << i)) != 0 {
                // print!("{}", (pat & (1 << i)));
                sum += *x;
                card_cnt += 1;
            }
        }
        if card_cnt != 0 && sum == a * card_cnt {
            // print!(" {:b} avg={}", pat, sum as f64 / card_cnt as f64);
            cnt += 1;
        }
        // println!();
    }
    println!("{}", cnt);
}
