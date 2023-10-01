use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        mut s: Chars,
    };
    s.insert(0, '-');
    let mut cnt = 1;
    let mut cnt_b = 1;
    for i in 1..=n {
        if s[i] == 'a' {
            // print!("a: cnt={cnt} ");
            if cnt <= a + b {
                println!("Yes");
                cnt += 1;
            } else {
                println!("No");
            }
        } else if s[i] == 'b' {
            // print!("b: ");
            if cnt <= a + b && cnt_b <= b {
                println!("Yes");
                cnt_b += 1;
                cnt += 1;
            } else {
                println!("No");
            }
        } else {
            // print!("c: ");
            println!("No");
        }
    }
}
