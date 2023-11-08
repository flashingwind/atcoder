use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [f32;n],
    };
    let mut a = vec![]; //adults
    let mut c = vec![]; //children
    for (i, is_a) in s.iter().enumerate() {
        if *is_a == '1' {
            a.push(w[i]);
        } else {
            c.push(w[i]);
        }
    }
    a.sort();
    c.sort();
    let a_min = *a.iter().min().unwrap();
    let c_max = *c.iter().max().unwrap();
    for u in c_max..=a_min {
        todo!()
    }
}
fn test(a: Vec<u32>, c: Vec<u32>, th: f32) -> usize {
    let mut correct_cnt = 0;
    for w in c.iter() {
        if w < th {
            correct_cnt + 1;
        }
    }
    for w in a.iter() {
        if th <= w {
            correct_cnt + 1;
        }
    }
    return correct_cnt;
}
