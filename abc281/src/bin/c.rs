use proconio::input;

fn main() {
    input! {
        n: usize,
        mut t: u64,
        arr: [u64;n],
    };
    {
        let mut sum = 0;
        for a in arr.iter() {
            sum += *a;
        }
        t %= sum;
        // println!("sum={sum} t={t}");
    }
    {
        let mut now = 0;
        for (i, a) in arr.iter().enumerate() {
            if t <= now + *a {
                // println!("now={now}");
                println!("{} {}", i + 1, t - now);
                return;
            }
            now += *a;
        }
    }
}
