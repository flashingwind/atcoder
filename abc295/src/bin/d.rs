use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut cnt = 0;
    for l in 0..s.len() {
        let mut ss = Vec::with_capacity(s.len());
        ss.push(s[l]);
        let mut is_ok = true;
        for r in l + 1..s.len() {
            ss.push(s[r]);
            if (r - l + 1) % 2 == 0 {
                if is_ok && 2 <= ss.len() && ss[ss.len() - 1] == ss[ss.len() - 2] {
                    is_ok = true;
                    ss.pop();
                    ss.pop();
                } else {
                    ss.sort();
                    is_ok = true;
                    for i in (0..ss.len() - 1).step_by(2) {
                        if ss[i] != ss[i + 1] {
                            is_ok = false;
                            break;
                        }
                    }
                }
                if is_ok {
                    cnt += 1;
                    ss.clear();
                }
                // println!("{r}-{l}: {is_ok} non_ok_count={non_ok_count}: {:?}", ss);
            }
        }
    }
    println!("{}", cnt);
}
