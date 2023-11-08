use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: usize,
        mut y: usize,
        mut z: usize,
        aa: [u32;n],
        bb: [u32;n],
    };
    let mut v: Vec<(u32, u32, usize)> = vec![];
    for i in 0..n {
        v.push((aa[i], bb[i], i));
    }
    let mut ok: Vec<(u32, u32, usize)> = vec![];
    v.reverse();
    v.sort_by(|a, b| a.0.cmp(&b.0));
    // println!("{:?}", v);
    while x != 0 {
        if let Some(v) = v.pop() {
            // println!("{:?}", v);
            ok.push(v);
        } else {
            print_vec(ok);
            return;
        }
        x -= 1;
    }
    v.sort_by(|a, b| b.2.cmp(&a.2));
    v.sort_by(|a, b| a.1.cmp(&b.1));
    while y != 0 {
        if let Some(v) = v.pop() {
            // println!("{:?}", v);
            ok.push(v);
        } else {
            print_vec(ok);
            return;
        }
        y -= 1;
    }
    v.sort_by(|a, b| b.2.cmp(&a.2));
    v.sort_by(|a, b| (a.0 + a.1).cmp(&(b.0 + b.1)));
    // println!(
    //     "{:?}",
    //     v.iter().map(|&v| (v.0 + v.1, v.2)).collect::<Vec<_>>()
    // );
    while z != 0 {
        if let Some(v) = v.pop() {
            // println!("{:?}", v);
            ok.push(v);
        } else {
            print_vec(ok);
            return;
        }
        z -= 1;
    }
    print_vec(ok);
}

fn print_vec(ok: Vec<(u32, u32, usize)>) {
    for n in ok.iter().sorted_by(|a, b| a.2.cmp(&b.2)) {
        println!("{}", n.2 + 1);
    }
}
