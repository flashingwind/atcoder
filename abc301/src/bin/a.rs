use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut cnt_taka = 0;
    let mut cnt_aoki = 0;
    let mut winner = 'N';
    for i in 0..n {
        if s[i] == 'T' {
            cnt_taka += 1;
        } else {
            cnt_aoki += 1;
        }
        if n < cnt_taka * 2 {
            println!("T");
            return;
        } else if n < cnt_aoki * 2 {
            println!("A");
            return;
        } else if cnt_aoki * 2 == n && winner == 'N' {
            winner = 'A';
        } else if cnt_taka * 2 == n && winner == 'N' {
            winner = 'T';
        }
    }
    if n == cnt_taka * 2 || n == cnt_aoki * 2 {
        println!("{}", winner);
    }
}
