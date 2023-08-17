use proconio::input;
/*
5
56
51
54
33
32

111000
110011
110110
100001
100000
 */
fn main() {
    input! {
        t: u64,
        ns: [u64;t],
    };
    for n in ns.iter() {
        let str = format!("{:b}", n);
        let cnt1 = str.chars().filter(|c| *c == '1').count();
        // println!("cnt1={cnt1} str1={str}");
        if str.len() <= 3 {
            if cnt1 == 3 {
                // println!("(1)str2={str}");
                println!("{}", n);
            } else {
                println!("-1");
            }
            continue;
        } else if 4 <= str.len() {
            if cnt1 == 1 {
                let str2 = format!("111{}", "0".repeat(str.len() - 4));
                // println!("str2={str2}");
                println!("{}", u128::from_str_radix(str2.as_str(), 2).unwrap());
            } else if cnt1 == 3 {
                // println!("str2=N={:b}", n);
                println!("{}", n);
            } else if cnt1 == 2 {
                let mut str2 = String::new();
                let mut cnt = 0;
                for (_i, c) in str.char_indices() {
                    if c == '1' {
                        cnt += 1;
                    }
                    str2 = format!("{}{}", str2, c);
                    if cnt == 2 {
                        if str.len() - str2.len() == 0 {
                            let str2 = format!("111{}", "0".repeat(str.len() - 4));
                            // println!("str2={str2}");
                            println!("{}", u128::from_str_radix(str2.as_str(), 2).unwrap());
                        } else {
                            str2 = format!("{}1{}", str2, &"0".repeat(str.len() - str2.len()));
                            println!("{}", u128::from_str_radix(str2.as_str(), 2).unwrap());
                        }
                        continue;
                    }
                }
            } else if 3 < cnt1 {
                let mut str2 = String::new();
                let mut cnt = 0;
                for (_i, c) in str.char_indices() {
                    if c == '1' {
                        cnt += 1;
                    }
                    str2 = format!("{}{}", str2, c);
                    if cnt == 3 {
                        println!(
                            "{}",
                            u128::from_str_radix(
                                format!("{}{}", str2, &"0".repeat(str.len() - str2.len())).as_str(),
                                2
                            )
                            .unwrap()
                        );
                        continue;
                    }
                }
            }
        } else {
            println!("-1");
        }
    }
}
