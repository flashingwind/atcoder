use proconio::input;

fn main() {
    input! {
        n: usize,
        // a: [u32;n],
    };
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        input! {
            ai: u32,
            bi: u32,
        };
        a.push(ai);
        b.push(bi);
    }
    let mut i_min_a = 0;
    for i in 0..n {
        if a[i] < a[i_min_a] {
            i_min_a = i;
        }
    }
    let mut i_min_b = n - 1;
    for i in 0..n {
        if b[i] < b[i_min_b] {
            i_min_b = i;
        }
    }
    if i_min_a == i_min_b {
        let cost_same = a[i_min_a] + b[i_min_a];
        let mut i_min_a2 = 0;
        let mut i_min_b2 = 0;
        for i in 0..n {
            if i_min_a != i {
                if b[i] < b[i_min_a2] {
                    i_min_a2 = i;
                }
                if a[i] < a[i_min_b2] {
                    i_min_b2 = i;
                }
            }
        }
        let cost_w_min_a = a[i_min_a].max(b[i_min_a2]);
        let cost_w_min_b = a[i_min_a].max(b[i_min_b2]);
        let min = cost_same.min(cost_w_min_a.min(cost_w_min_b));
        // println!("{} {} {}", cost_same, cost_w_min_a, cost_w_min_b);
        println!("{}", min);
    } else {
        // println!("{},{}", a[i_min_a], b[i_min_b]);
        println!("{}", a[i_min_a].max(b[i_min_b]));
    }
}
