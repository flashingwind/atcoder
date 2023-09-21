// use std::collections::VecDeque;
use proconio::fastout;
use proconio::{input, marker::Chars};

#[fastout]
fn main() {
    input! {
        _: usize,
        num_q: usize,
        // mut s_raw: String,
        mut s: Chars,
        q_vec: [(i32, usize);num_q],
    };
    // let mut s = s_raw.chars().collect::<VecDeque<char>>();
    // let mut itr = s.iter();
    let mut i = 0;
    // println!("s={:?}", s);
    for q in q_vec {
        if q.0 == 1 {
            rot(&mut i, q.1, s.len());
        } else if q.0 == 2 {
            prt(i, q.1, &s);
        }
    }
}

#[inline(always)]
fn rot(i: &mut usize, diff: usize, s_len: usize) {
    // print!("ROT {}->", i);
    if *i < diff {
        *i = s_len + *i - diff;
    } else {
        *i -= diff;
    }
    // println!("{}", i);
}

#[inline(always)]
fn prt(i: usize, diff: usize, s: &Vec<char>) {
    let j = (i + diff - 1) % s.len();
    // print!("PRT s{}=", j);
    println!("{}", s[j]);
}
