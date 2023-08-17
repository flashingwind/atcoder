use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        c: [usize;n],
    };
    let mut map = vec![Vec::new(); m + 1];
    for i in 0..n {
        map[c[i]].push(i);
    }
    // println!("{:?}", map);
    for clr in 1..=m {
        let tmp = map[clr].pop().unwrap();
        map[clr].reverse();
        map[clr].push(tmp);
    }
    // println!("{:?}", map);
    for i in 0..n {
        let j = map[c[i]].pop().unwrap();
        // print!("{}", j);
        print!("{}", s[j]);
    }
    println!();
}
