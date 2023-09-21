use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut cnt_max = 0usize;
    let mut found_bar_a = false;
    let mut found_bar_b = false;
    let mut is_inside_dango = false;
    let mut cnt = 0;
    for i in 0..n {
        if s[i] == '-' {
            found_bar_a = true;
            if is_inside_dango {
                found_bar_b = true;
            }
            if is_inside_dango && (found_bar_a || found_bar_b) && cnt_max < cnt {
                cnt_max = cnt;
            }
            is_inside_dango = false;
        } else if i == n - 1 {
            if !is_inside_dango && s[i] == 'o' {
                cnt = 1;
                is_inside_dango = true;
                found_bar_b = false;
            } else if s[i] == 'o' {
                cnt += 1;
                is_inside_dango = true;
                found_bar_b = false;
            }
            if is_inside_dango && found_bar_a && cnt_max < cnt {
                cnt_max = cnt;
            }
            is_inside_dango = false;
        } else if !is_inside_dango && s[i] == 'o' {
            cnt = 1;
            is_inside_dango = true;
        } else if s[i] == 'o' {
            cnt += 1;
            is_inside_dango = true;
        }
        if is_inside_dango && found_bar_a && cnt_max < cnt {
            cnt_max = cnt;
        }
        // println!("{i}:{} cnt={cnt} cnt_max={cnt_max} is_inside_dango={is_inside_dango} bar_a={found_bar_a}", s[i]);
    }
    if cnt_max != 0 {
        println!("{}", cnt_max);
    } else {
        println!("-1");
    }
}
