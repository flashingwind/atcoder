use proconio::input;

fn main() {
    input! {
        max: i64,
        num_charges: usize,
        t: usize,
        charges: [(usize,usize);num_charges],
    };
    let mut bat = max;
    let mut itr = charges.iter();
    let mut next = *itr.next().unwrap();
    let mut tt = 0;
    // println!("0:{bat}");
    while tt <= t {
        let (c_start, c_stop) = next;
        if tt < c_start {
            bat -= (c_start - tt) as i64;
            if bat <= 0 {
                println!("No");
                return;
            }
            tt = c_start;
        } else if c_start == tt {
            bat += (c_stop - c_start) as i64;
            if max <= bat {
                bat = max;
            }
            tt = c_stop;
        } else {
            if let Some(tmp) = itr.next() {
                next = *tmp;
            } else {
                bat -= (t - tt) as i64;
                tt = t;
                break;
            }
        }
        // println!("{tt}:{c_start}--{c_stop}");
        // println!("  bat={bat}");
    }
    // println!("{tt}:fin. {bat}");
    if 0 < bat - (t - tt) as i64 {
        println!("Yes");
    } else {
        println!("No");
    }
}
