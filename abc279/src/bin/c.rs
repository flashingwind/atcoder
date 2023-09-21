use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        map_s: [Chars;h],
        map_t: [Chars;h],
    };

    // transpose
    let mut ss = vec![String::with_capacity(h); w];
    let mut st = vec![String::with_capacity(h); w];
    for i in 0..h {
        for j in 0..w {
            ss[j] += map_s[i][j].to_string().as_str();
            st[j] += map_t[i][j].to_string().as_str();
        }
    }
    let mut is_all_ok = true;
    for (j1, ss_i) in ss.iter().enumerate() {
        let mut is_ok = false;
        for (j2, st_i) in st.iter().enumerate().skip(j1) {
            if *ss_i == *st_i {
                is_ok = true;
                st.swap(j1, j2);
                break;
            }
        }
        if !is_ok {
            is_all_ok = false;
            break;
        }
    }
    if is_all_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
