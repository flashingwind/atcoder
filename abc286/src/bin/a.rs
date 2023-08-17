use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
        mut a: [i32;n],
    };
    a.insert(0, 0);
    let mut b = a.to_owned();
    for i in 1..p {
        b[i] = a[i];
    }
    // println!("{:?}", b);
    for i in p..=q {
        b[i] = a[i - p + r];
    }
    // println!("{:?}", b);
    for i in r..=s {
        b[i] = a[i - r + p];
    }
    // println!("{:?}", b);
    print!("{}", b[1]);
    for i in 2..=n {
        print!(" {}", b[i]);
    }
    println!();
}
