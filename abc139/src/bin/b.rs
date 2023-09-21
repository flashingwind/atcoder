use proconio::input;

fn main() {
    input! {
        num_branch: u32,
        target: u32,
    };
    let mut sum = 1;
    let mut cnt = 0;
    while sum < target {
        sum += num_branch - 1;
        cnt += 1;
    }
    println!("{}", cnt);
}
