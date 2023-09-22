use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize;n],
        bb: [usize;m],
    }
    let len_max = *aa.last().unwrap().max(bb.last().unwrap());
    let mut a = vec![false; len_max + 1];
    let mut b = vec![false; len_max + 1];

    for i in 0..n {
        a[aa[i]] = true;
    }
    for j in 0..m {
        b[bb[j]] = true;
    }
    // println!("{:?}", a);
    // println!("{:?}", b);
    for i in 0..=len_max {
        if (!a[i] || !b[i]) && !(!a[i] && !b[i]) {
            if i == len_max {
                print!("{}\n", i);
            } else {
                print!("{} ", i);
            }
        }
    }
}
