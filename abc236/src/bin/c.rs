use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String;n],
        t: [String;m],
    };
    let mut itr_exp = t.iter();
    let mut exp_sta = itr_exp.next().unwrap();
    for sta in s.iter() {
        // println!("{},{}", sta, exp_sta);
        if *sta == *exp_sta {
            if let Some(tmp) = itr_exp.next() {
                exp_sta = tmp;
            }
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
