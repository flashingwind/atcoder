use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    };
    let mut burg: VecDeque<char> = VecDeque::with_capacity(512 * 1024 * 1024);
    burg.insert(0, 'P');
    //burg.insert(0, 'P');
    for _ in 1..=n {
        let mut burg_old = burg.to_owned();
        burg.push_front('B');
        burg.push_back('P');
        burg.append(&mut burg_old);
        burg.push_back('B');
    }
    //println!("burg={:?}", burg);
    let mut p_eated = 0;
    for i in 0..x {
        if burg[i] == 'P' {
            p_eated += 1;
        }
    }
    println!("{}", p_eated);
}
