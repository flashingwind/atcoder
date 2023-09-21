use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut s: Chars,
    };
    let mut is_changed = vec![false; n];
    for i in 1..n - 1 {
        if s[i - 1] == 'Y' && s[i] == 'X' && s[i + 1] == 'Y' && !is_changed[i] {
            s[i] = 'Y';
            is_changed[i] = true;
            k -= 1;
            if k == 0 {
                let mut cnt = 0;
                for i in 1..n {
                    if s[i - 1] == 'Y' && s[i] == 'Y' {
                        cnt += 1;
                    }
                }
                println!("{}", cnt);
                return;
            }
        }
    }
    for i in 1..n {
        if s[i - 1] == 'Y' && s[i] == 'X' && !is_changed[i] {
            s[i] = 'Y';
            is_changed[i] = true;
            k -= 1;
            if k == 0 {
                let mut cnt = 0;
                for i in 1..n {
                    if s[i - 1] == 'Y' && s[i] == 'Y' {
                        cnt += 1;
                    }
                }
                println!("{}", cnt);
                return;
            }
        }
    }
    for i in 1..n {
        if s[i - 1] == 'X' && s[i] == 'Y' && !is_changed[i - 1] {
            s[i - 1] = 'Y';
            is_changed[i - 1] = true;
            k -= 1;
            if k == 0 {
                let mut cnt = 0;
                for i in 1..n {
                    if s[i - 1] == 'Y' && s[i] == 'Y' {
                        cnt += 1;
                    }
                }
                println!("{}", cnt);
                return;
            }
        }
    }
    for i in 0..n {
        if s[i] == 'X' && !is_changed[i] {
            s[i] = 'Y';
            is_changed[i - 1] = true;
            k -= 1;
            if k == 0 {
                let mut cnt = 0;
                for i in 1..n {
                    if s[i - 1] == 'Y' && s[i] == 'Y' {
                        cnt += 1;
                    }
                }
                println!("{}", cnt);
                return;
            }
        }
    }

    // kがあまった
    k %= 2;
    for i in 1..n {
        // println!("k={k} i={i} s={:?}", s);
        if s[i] == 'Y' && !is_changed[i] {
            s[i] = 'X';
            is_changed[i] = true;
            k -= 1;
        }
        if k == 0 {
            let mut cnt = 0;
            for i in 1..n {
                if s[i - 1] == 'Y' && s[i] == 'Y' {
                    cnt += 1;
                }
            }
            println!("{}", cnt);
            return;
        }
    }
    if k != 0 {
        for i in 0..n {
            // println!("k={k} i={i} s={:?}", s);
            if s[i] == 'Y' {
                s[i] = 'X';
                k -= 1;
            }
            if k == 0 {
                let mut cnt = 0;
                for i in 1..n {
                    if s[i - 1] == 'Y' && s[i] == 'Y' {
                        cnt += 1;
                    }
                }
                println!("{}", cnt);
                return;
            }
        }
    }

    let mut cnt = 0;
    for i in 1..n {
        if s[i - 1] == 'Y' && s[i] == 'Y' {
            cnt += 1;
        }
    }
    println!("ERR: cnt={} k={}", cnt, k);
}
