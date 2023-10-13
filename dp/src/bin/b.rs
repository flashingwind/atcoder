use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [u64;n],
    };
    let mut cost = vec![u64::MAX; n];
    cost[0] = 0;
    cost[1] = h[1].abs_diff(h[0]);
    for i in 1..n {
        let di_start;
        if k <= i {
            di_start = k;
        } else {
            di_start = i;
        }
        cost[i] = cost[i - di_start] + h[i].abs_diff(h[i - di_start]);
        for di in (1..=di_start).rev() {
            // println!(
            //     "i{i}-di{di}={} cost[i-di]={} diff={}",
            //     i - di,
            //     cost[i - di],
            //     h[i].abs_diff(h[i - di])
            // );
            cost[i] = cost[i].min(cost[i - di] + h[i].abs_diff(h[i - di]));
        }
    }
    // println!("{:?}", cost);
    println!("{}", cost.last().unwrap());
}
