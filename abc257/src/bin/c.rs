use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [f32;n],
    };
    let mut a = vec![];
    let mut c = vec![];
    for (i, is_a) in s.iter().enumerate() {
        if *is_a == '1' {
            a.push(w[i]);
        } else {
            c.push(w[i]);
        }
    }
    let a_min = *a.iter().min().unwrap();
    let c_max = *c.iter().max().unwrap();
    for u in c_max..=a_min {
        todo!()
    }
}
fn cnt_correct(a: Vec<u32>, c: Vec<u32>, x: f32) -> usize {
    let mut cnt = 0;
    return 0;
}
