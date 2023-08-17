use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut n_vec: Chars,
    };
    let n_max_str: String = n_vec.iter().collect();
    let n_max = n_max_str.parse::<u128>().unwrap_or(0);
    if n_vec.len() == 1 {
        println!("0");
        return;
    }
    let n_lower_max = "9".repeat(n_vec.len() / 2).parse().unwrap_or(0);
    if n_lower_max == 0 {
        println!("0");
        return;
    }
    let mut cnt = 0;
    // println!("n_lower_max={n_lower_max}");
    for n_lower in 1..=n_lower_max {
        let ans = n_lower.to_string().repeat(2).parse::<u128>().unwrap_or(0);
        if ans <= n_max {
            // println!("ans={ans}");
            cnt += 1;
        } else {
            break;
        }
    }
    println!("{}", cnt);
}
