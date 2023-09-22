use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [(usize,usize,usize);n],
    }
    let mut taka = Vec::new();
    let mut sum_taka = 0;
    let mut aoki = Vec::new();
    let mut sum_aoki = 0;
    for i in 0..n {
        if a[i].0 < a[i].1 {
            taka.push((i, (a[i].1 - a[i].0) / 2, a[i].2));
            sum_taka += a[i].2;
        } else {
            aoki.push((i, (a[i].0 - a[i].1) / 2, a[i].2));
            sum_aoki += a[i].2;
        }
    }
    if sum_aoki < sum_taka {
        println!("0");
    } else {
        aoki.sort_by(|b, c| (b.2 / b.1).cmp(&(c.2 / c.1)));
        aoki.reverse();
        let mut cost = 0;

        for i in 0..n {
            let v = aoki.pop().unwrap();
            println!("{cost} T{sum_taka}:A{sum_aoki}");
            cost += v.1;
            sum_taka += v.2;
            sum_aoki -= v.2;
            println!("  {cost} T{sum_taka}:A{sum_aoki}");
            if n / 2 < sum_taka {
                break;
            }
        }
        println!("{}", cost);
    }
}
