use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let s_rnc = r_l_encode(s);
    let t_rnc = r_l_encode(t);
    if s_rnc.len() != t_rnc.len() {
        println!("No");
        return;
    }
    let n = s_rnc.len();
    let mut s_itr = s_rnc.iter();
    let mut t_itr = t_rnc.iter();
    for _i in 0..n {
        if let Some(&(c1, l1)) = s_itr.next() {
            if let Some(&(c2, l2)) = t_itr.next() {
                // println!("{_i}: l1:{c1}:{} != l2:{c2}:{}", l1, l2);
                if c1 == c2 && l1 == l2 {
                } else if c1 == c2 && (l1 != l2 && 2 <= l1) {
                } else {
                    println!("No");
                    return;
                }
            } else {
                println!("No");
                return;
            }
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn r_l_encode(s: Vec<char>) -> Vec<(char, usize)> {
    let mut runs = vec![];
    for c in s.iter() {
        if let Some((r, l)) = runs.last_mut() {
            if *r == *c {
                *l += 1;
            } else {
                runs.push((*c, 1));
            }
        } else {
            runs.push((*c, 1));
        }
    }
    return runs;
}
