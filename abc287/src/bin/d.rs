use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let mut sp = Vec::with_capacity(t.len());
    let mut last_is_eq = false;
    for x in 0..=t.len() {
        sp.clear();
        sp.extend(s[0..x].iter());
        // println!("sp_i={:?}", sp);
        sp.extend(s[(s.len() - (t.len() - x))..s.len()].iter());
        // println!(
        //     "sp_i={:?} ={:?}+{:?}",
        //     sp,
        //     &s[0..x],
        //     &s[(s.len() - (t.len() - x))..s.len()]
        // );
        // println!("t   ={:?}", t);
        // println!("sp   ={:?}", sp);
        let mut is_eq = true;
        for (i, t_i) in t.iter().enumerate() {
            if i <= x {
                // println!("{}: {}=={}:", i, *t_i, sp[i]);
                if *t_i == sp[i] {
                } else if *t_i == '?' || sp[i] == '?' {
                } else {
                    is_eq = false;
                    break;
                }
            } else if last_is_eq {
                is_eq = last_is_eq;
            }
        }
        if is_eq {
            println!("Yes");
        } else {
            println!("No");
        }
        last_is_eq = is_eq;
    }
}
