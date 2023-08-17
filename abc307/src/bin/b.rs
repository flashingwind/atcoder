use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars;n],
    };
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            // let tmp = format("{}{}", a[i], a[j]);
            let mut s = a[i].to_owned();
            s.extend(a[j].iter());
            // println!("s{i}s{j}={:?}", s);
            if is_kaibun(&s) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

fn is_kaibun(s: &Vec<char>) -> bool {
    for i in 0..=s.len() / 2 {
        // println!(
        //     "s{i}:{} -- s{}: {}",
        //     s[i],
        //     s.len() - 1 - i,
        //     s[s.len() - 1 - i]
        // );
        if s[i] != s[s.len() - 1 - i] {
            return false;
        }
    }
    return true;
}
