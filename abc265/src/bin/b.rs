use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut t: i64,
        mut spend_time: [i64;n-1],
        mut bonus: [(usize,i64);m],
    };
    spend_time.insert(0, 0);
    bonus.insert(0, (0, 0));
    for (i, b) in bonus.iter() {
        spend_time[*i] -= *b;
    }
    for t_spend in spend_time.iter().take(n).skip(1) {
        if 0 < t - *t_spend {
            t -= *t_spend;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
