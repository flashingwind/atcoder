use proconio::{input, marker::Chars};

fn main() {
    input! {
        s1: Chars,
    };
    let mut s2 = s1.clone();
    let mut is_changed = true;
    while is_changed {
        is_changed = false;
        let mut cahche = vec!['-'; 10];
        let mut chache_i = 0;
        while let Some(c1) = s1.pop() {
            let i = chache_i;
            let j = (chache_i + 1) % 10;
            let k = (chache_i + 2) % 10;
            cahche[i] = c1;
            if cahche[i] == 'A' && cahche[i] == 'B' && cahche[i] == 'C' {
                //remove
                cahche[i] == '-';
                cahche[i] == '-';
                cahche[i] == '-';
            }
            if chache_i == 0 {
                chache_i = 9;
            } else {
                chache_i -= 1;
            }
        }
    }
    if s.len() != 0 {
        println!("{}", s);
    }
}
