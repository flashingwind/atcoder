use proconio::{input, marker::Usize1};
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        num_q: usize,
    };
    let mut d = VecDeque::new();
    let mut dir: HashMap<char, (isize, isize)> = HashMap::new();
    dir.insert('R', (1, 0));
    dir.insert('L', (-1, 0));
    dir.insert('U', (0, 1));
    dir.insert('D', (0, -1));

    for i in 0..n as isize {
        d.push_back((i + 1, 0isize));
    }
    for _t in 0..num_q {
        input! {
            t: u32,
        };
        if t == 1 {
            input! {
                c: char,
            };
            // println!("{c}:{:?}", d);
            let mut tmp = d.pop_back().unwrap();
            let dxdy = dir.get(&c).unwrap();
            tmp.0 = d[0].0 + dxdy.0;
            tmp.1 = d[0].1 + dxdy.1;
            d.push_front(tmp);
            // println!("->{:?}", d);
        } else {
            input! {
                ip: Usize1,
            };
            println!("{} {}", d.get(ip).unwrap().0, d.get(ip).unwrap().1);
        }
    }
}
