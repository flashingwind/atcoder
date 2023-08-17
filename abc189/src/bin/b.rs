use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u32,
        sake: [(u32,u32);n],
    };
    let mut sum_alc = 0;
    for (i, (vol, alc)) in sake.iter().enumerate() {
        sum_alc += vol * alc;
        if x * 100 < sum_alc {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
