use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        max: u64,
        num_sets: usize,
    };
    let mut sets = Vec::new();
    for _ in 0..num_sets {
        input! {
            c: usize,
            set: [u64;c],
        };
        sets.push(set.to_owned());
    }
    let mut cnt = 0;
    for x in 1..=max {
        //M 個の集合から 1個以上の集合を選ぶ
        // 合計n個
        println!("x={x}");
        for num_sets_local in 1..=num_sets {
            println!("n={num_sets_local}");
            // nPn
            for pat in (0..num_sets_local).combinations(num_sets_local) {
                for i in pat.iter() {
                    if sets[*i].contains(&x) {
                        print!("pat={:?}", pat);
                        cnt += 1;
                        break;
                    }
                }
                println!();
            }
        }
    }
    println!("{}", cnt);
}
