use proconio::input;

fn main() {
    input! {
        m: usize,
        d: [usize;m],
    }

    let mut sum = 0;
    for mon in 1..=m {
        for day in 1..=d[mon - 1] {
            sum += 1;
            // println!(" {} {}", mon, day);
            if sum == (d.iter().sum::<usize>() + 1) / 2 {
                println!("{} {}", mon, day);
                return;
            }
        }
    }
}
