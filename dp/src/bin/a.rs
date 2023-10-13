use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u32;n],
    };
    let mut cost = vec![u32::MAX; n];
    cost[0] = 0;
    cost[1] = h[1].abs_diff(h[0]);
    for i in 2..n {
        cost[i] =
            (cost[i - 1] + h[i].abs_diff(h[i - 1])).min(cost[i - 2] + h[i].abs_diff(h[i - 2]));
    }
    // println!("{:?}", cost);
    println!("{}", cost.last().unwrap());
}
