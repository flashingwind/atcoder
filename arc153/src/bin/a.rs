use proconio::input;

fn main() {
    input! {
        n: u32,
    };
    println!("{}", calc(n));
}
fn calc(n: u32) -> String {
    let mut cnt = 1;
    let mut str: String = "".to_string();
    for s1 in 1..=9 {
        let s2 = s1;
        for s3 in 0..=9 {
            for s4 in 0..=9 {
                for s5 in 0..=9 {
                    let s6 = s5;
                    for s7 in 0..=9 {
                        for s8 in 0..=9 {
                            let s9 = s7;
                            if cnt >= n {
                                str = format!(
                                    "{}{}{}{}{}{}{}{}{}",
                                    s1, s2, s3, s4, s5, s6, s7, s8, s9
                                );
                                //println!("{}{}{}{}{}{}{}{}{}", s1, s2, s3, s4, s5, s6, s7, s8, s9);
                                return str;
                            }
                            cnt += 1;
                        }
                    }
                }
            }
        }
    }
    //println!("cnt={} n={} str={}", cnt, n, str);
    return str;
}
