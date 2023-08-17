use proconio::input;

fn main() {
    input! {
        x: i32,
        n: usize,
        mut p: [i32;n],
    };
    let mut min_v_ofmin_diff = x;
    let mut min_diff = std::i32::MAX;
    if n == 0 {
        println!("{}", min_v_ofmin_diff);
        return;
    }
    p.sort();
    for pp in 0..=101 {
        if !p.contains(&pp) {
            if (pp - x).abs() == min_diff {
                if pp < min_v_ofmin_diff {
                    min_v_ofmin_diff = pp;
                }
            } else if (pp - x).abs() <= min_diff {
                min_diff = (pp - x).abs();
                min_v_ofmin_diff = pp;
            }
            // println!(
            //     "pp={pp} diff={} min_diff={min_diff} min_v={min_v_ofmin_diff}",
            //     (pp - x).abs()
            // );
        }
    }
    println!("{}", min_v_ofmin_diff);
}
