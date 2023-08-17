use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
    };
    let mut is_exists = vec![0usize; 9];
    for aa in a {
        if aa <= 399 {
            is_exists[0] += 1;
        } else if aa <= 799 {
            is_exists[1] += 1;
        } else if aa <= 1199 {
            is_exists[2] += 1;
        } else if aa <= 1599 {
            is_exists[3] += 1;
        } else if aa <= 1999 {
            is_exists[4] += 1;
        } else if aa <= 2399 {
            is_exists[5] += 1;
        } else if aa <= 2799 {
            is_exists[6] += 1;
        } else if aa <= 3199 {
            is_exists[7] += 1;
        } else {
            is_exists[8] += 1;
        }
    }
    let cnt_exist_color: usize = is_exists.iter().take(8).filter(|v| 0 < **v).count();
    println!(
        "{} {}",
        cnt_exist_color.max(1),
        cnt_exist_color + is_exists[8]
    );
}
