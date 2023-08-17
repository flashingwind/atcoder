use proconio::input;

fn main() {
    input! {
        n: usize,
        mut next: [usize;n],
    };
    next.insert(0, 0);
    let mut cnt = 0;
    let mut i = 1;
    while cnt <= n {
        if i == 2 {
            println!("{}", cnt);
            return;
        } else if next[i] == i {
            println!("-1");
            return;
        }
        i = next[i];
        cnt += 1;
    }
    if n < cnt {
        println!("-1");
    }
}
