use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String;n],
        t: [String;m],
    };
    let mut cnt = 0;
    for i in 0..n {
        for j in 0..m {
            if let Some(v) = s[i].rfind(t[j].as_str()) {
                // println!("{i},{j}: {v}");
            }
            if s[i].rfind(t[j].as_str()) == Some(3) {
                cnt += 1;
                break;
            }
        }
    }
    println!("{}", cnt);
}
