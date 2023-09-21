use proconio::input;

fn main() {
    input! {
        n_max: u32,
    };
    let mut cnt = 0;
    for n in 1..=n_max {
        if !format!("{:o}", n).contains("7") && !format!("{}", n).contains("7") {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
