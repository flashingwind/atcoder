use proconio::input;

fn main() {
    input! {
        x_max: usize,
    };
    let mut is_checked = vec![false; x_max + 6 + 100];
    is_checked[0] = true;
    for x in 0..x_max {
        if is_checked[x] {
            for d in 0..6 {
                is_checked[x + d + 100] = true;
            }
        }
    }
    if is_checked[x_max] {
        println!("1");
    } else {
        println!("0");
    }
}
