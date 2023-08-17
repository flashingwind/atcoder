use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        num_q: usize,
    };
    let mut q = VecDeque::new();
    for _ in 0..num_q {
        input! {
            typ: u32,
        };
        if typ == 1 {
            input! {
                x: u64,
                c: u64,
            };
            q.push_front((x, c));
        } else {
            // typ==2
            input! {
                mut c_inp: u64,
            };
            let mut sum = 0;
            while 0 < c_inp {
                let (x, mut c_poped) = q.pop_back().unwrap();
                if c_inp <= c_poped {
                    sum += x * c_inp;
                    c_poped -= c_inp;
                    q.push_back((x, c_poped));
                    break;
                } else {
                    sum += x * c_poped;
                    c_inp -= c_poped;
                }
            }
            println!("{}", sum);
        }
    }
}
