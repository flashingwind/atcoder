use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32;n],
        b: [i32;n],
    };
    let mut cnt_eq_same_i = 0;
    let mut cnt_eq_diff_i = 0;
    for (i, ai) in a.iter().enumerate() {
        for (j, bi) in b.iter().enumerate() {
            if ai == bi {
                if i == j {
                    cnt_eq_same_i += 1;
                } else {
                    cnt_eq_diff_i += 1;
                }
            }
        }
    }
    println!("{}\n{}", cnt_eq_same_i, cnt_eq_diff_i);
}
