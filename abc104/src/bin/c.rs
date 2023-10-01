use proconio::input;

fn main() {
    input! {
        d: usize,
        g: usize,
    };
    let mut p = vec![];
    let mut c = vec![];
    let mut total = vec![];
    for i in 0..d {
        input! {
            pi: usize,
            ci: usize,
        };
        p.push(pi);
        c.push(ci);
        total.push((pi * 100 * i + ci, i));
    }
    total.sort();
    let mut cnt = 0;
    let mut sum = 0;
    let mut i_set = vec![];
    for i in 0..d {
        let j = total[i].1;
        sum += total[i];
        cnt += p[j];
        i_set.push(j);
        if g == sum {
            println!("{}", i);
            break;
        } else if g < sum {
            let mut rm_cnt = 0;
            let mut max_rm_cnt = usize::MAX;
            let mut dp = vec![];
            for k in i_set.iter() {
                dp[rm_cnt] = dp[rm_cnt].min(c[k] + 100 * k);
                if g < sum - c[j] - 100 * j {}
            }
            println!("{}", i);
            break;
        }
    }
}
