use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ice: [(usize,u64);n],
    }
    let mut max = 0;
    for k in 0..1 {
        ice.sort_by(|a, b| a.1.cmp(&b.1));
        ice.reverse();
        let max_v = ice[k].1;
        let max_v_cat = ice[k].0;
        ice.remove(k);
        // println!("ice[0].1 {}", ice[0].1);
        for i in 0..n - 1 {
            ice[i].1 = if ice[i].0 == max_v_cat {
                ice[i].1 / 2
            } else {
                ice[i].1
            };
        }
        ice.sort_by(|a, b| a.1.cmp(&b.1));
        ice.reverse();
        for i in 0..n - 1 {
            // println!("{}+{}={}", max_v, ice[i].1, max_v + ice[i].1);
            if max < max_v + ice[i].1 {
                max = max_v + ice[i].1;
            }
            if ice[i].1 < ice[0].1 / 2 {
                break;
            }
        }
        ice.insert(0, (max_v_cat, max_v));
    }
    println!("{}", max);
}
