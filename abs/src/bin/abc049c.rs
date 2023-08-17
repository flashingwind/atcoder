use proconio::input;

fn main() {
    let word = [
        "dreamer".to_string(),
        "dream".to_string(),
        "eraser".to_string(),
        "erase".to_string(),
    ];
    const D_MAX: usize = std::usize::MAX;
    //const D_MAX: usize = 100;
    let mut stack_i: Vec<usize> = Vec::new();
    input! {
        mut s: String,
    };
    let mut i_start: usize = 0;
    for _t in 0..D_MAX {
        let mut is_matched = false;
        //println!("{}: now s:{} i_start={}", _t, s, i_start);
        for (ii, w) in word[i_start..word.len()].iter().enumerate() {
            let i = ii + i_start;
            //println!("{}: chcek w:{}", _t, *w);
            if s.starts_with(w) {
                stack_i.push(i);
                s = s.replacen(w, "", 1);
                //println!("        SUC {} + {} i={}", w, s, i);
                is_matched = true;
                if s.is_empty() {
                    println!("YES");
                    return;
                }
            } else {
                //println!("        :FALSE")
            }
        }
        //back
        if !is_matched {
            if let Some(i) = stack_i.pop() {
                i_start = i + 1;
                //println!("NG,back s:←{}+{} i_start={}", word[i], s, i_start);
                s = [word[i].clone(), s].concat();
            } else {
                println!("NO");
                return;
            }
        }
    }
    println!("NO");
}
