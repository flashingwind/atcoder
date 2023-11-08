use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [u32;n],
    };
    //count
    let mut perm_cnt = 0;
    for i in 0..n {
        for l in 1..p[i] {
            perm_cnt += fact(p[i] - 1);
        }
    }

    perm_cnt -= 1;

    //generate the pattern before
    let mut s = vec![];
    for i in 1..=n as u32 {
        s.push((perm_cnt % i + 1).to_string());
        perm_cnt /= i;
    }
    s.reverse();
    println!("{}", s.join(" "));
}

fn fact(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        fact(n - 1) * n
    }
}
