use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    let mut cnt = 0;
    let mut zero_cnt = 0;
    for s_i in s {
        let d = s_i.to_digit(10).unwrap();
        if d == 0 && zero_cnt == 0 {
            zero_cnt = 1;
            cnt += 1;
            // print!("Z,");
        } else if d == 0 && zero_cnt == 1 {
            zero_cnt = 0;
            // print!("ZZ,")
        } else {
            zero_cnt = 0;
            cnt += 1;
            // print!("{},", d);
        }
    }
    // println!();
    println!("{}", cnt);
}
