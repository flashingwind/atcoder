use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        k: usize,
    };

    // let mut cnt = 0usize;
    let mut n321 = vec![];
    for num_d in 1..=10 {
        for pat in (0..=9).combinations(num_d) {
            let mut sum: u64 = 0;
            for &d in pat.iter().sorted().rev() {
                sum *= 10;
                sum += d;
            }
            // println!("  {sum}");
            n321.push(sum);
            // cnt += 1;
            // if cnt == k {
            //     println!("{sum}");
            //     return;
            // }
        }
    }
    n321.sort();
    // println!("    {cnt}=={}", n321.len());
    println!("{}", n321[k]);
}
