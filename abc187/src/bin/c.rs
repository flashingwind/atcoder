use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        str: [String;n],
    };
    let mut cache: BTreeMap<String, usize> = BTreeMap::new();
    for (_, t) in str.iter().enumerate() {
        *cache.entry(t.to_owned()).or_insert(0) += 1;
    }

    for (s_a, &cnt) in cache.to_owned().iter() {
        if !s_a.contains("!") {
            let s_b = &String::from(format!("!{}", s_a.to_owned()));
            let cnt_b = *cache.entry(s_b.to_owned()).or_insert(0);
            // println!("s_a={s_a} s_b={s_b} cnt={cnt} cnt_b={cnt_b}");

            if 1 <= cnt && 1 <= cnt_b {
                println!("{}", s_a);
                return;
            }
        }
    }
    println!("satisfiable");
}
