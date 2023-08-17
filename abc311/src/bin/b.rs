use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        d: usize,
        s: [Chars;n],
    };
    // println!("s:{:?}", s);
    let mut day_free = Vec::new();
    let mut cnt = 0;
    let mut cnt_max = 0;
    for day in 0..d {
        let mut is_free = true;
        for i in 0..n {
            if s[i][day] == 'x' {
                is_free = false;
                break;
            }
        }
        if is_free {
            if let Some(day_before) = day_free.last() {
                if day_before + 1 == day {
                    cnt += 1;
                } else {
                    cnt = 1;
                }
            } else {
                cnt = 1;
            }
            if cnt_max < cnt {
                cnt_max = cnt;
            }
            day_free.push(day);
        }
        // println!("cnt={cnt} day_free={:?}", day_free);
    }
    println!("{}", cnt_max);
}
