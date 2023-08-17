use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u32;n],
        b: [u32;m],
    };
    let mut c_index = vec![(0u32, 0usize, 'a'); n + m];
    let mut c = a.to_owned();
    c.extend(b.iter());
    for (i, ci) in c.iter().enumerate() {
        if i < n {
            c_index[i] = (*ci, 0, 'a');
        } else {
            c_index[i] = (*ci, 0, 'b');
        }
    }
    for (i, ci) in c_index.iter_mut().sorted().enumerate() {
        ci.1 = i + 1;
        // print!("cind{}={} ", ci.1, ci.0);
    }
    // println!("{:?}", c_index);
    let mut is_first = true;
    for ci in c_index.iter().take(n) {
        if !is_first {
            print!(" ");
        }
        print!("{}", ci.1);
        is_first = false;
    }
    println!();
    is_first = true;
    for ci in c_index.iter().skip(n) {
        if ci.2 == 'b' {
            if !is_first {
                print!(" ");
            }
            print!("{}", ci.1);
            is_first = false;
        }
    }
    println!();
}
