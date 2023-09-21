use proconio::input;

fn main() {
    input! {
        mut s: String,
    };
    let pat = vec!["dreamer", "dream", "eraser", "erase"];
    loop {
        let mut is_changed = false;
        for p in pat.iter() {
            let s_new = s.trim_end_matches(*p);
            if s != s_new {
                s = s_new.to_string();
                is_changed = true;
            }

            if s.is_empty() {
                println!("YES");
                return;
            }
        }
        if !is_changed {
            println!("NO");
            return;
        }
    }
}
