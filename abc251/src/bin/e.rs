use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
    };
    let mut costs: BTreeMap<(usize, usize), u32> = BTreeMap::new();
    for (a1, aa) in a.iter().enumerate() {
        let a2 = (a1 + 1) % (n - 1);
        costs.entry((a1, a2)).or_insert(*aa);
    }
    let mut sorted: Vec<(&(usize, usize), &u32)> = costs.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted.reverse();
    // println!("{:?}", sorted);

    let mut is_feeded = vec![false; n];
    let mut sum_cost = 0;
    let mut is_changed = true;
    while is_feeded.contains(&false) && is_changed {
        is_changed = false;
        if let Some(((a1, a2), c)) = sorted.pop() {
            if !is_feeded[*a1] && !is_feeded[*a2] {
                is_feeded[*a1] = true;
                is_feeded[*a2] = true;
                sum_cost += c;
                is_changed = true;
            }
        }
    }
    while is_feeded.contains(&false) {
        if let Some(((a1, a2), c)) = sorted.pop() {
            if !is_feeded[*a1] || !is_feeded[*a2] {
                is_feeded[*a1] = true;
                is_feeded[*a2] = true;
                sum_cost += c;
            }
        }
    }
    println!("{}", sum_cost);
}
