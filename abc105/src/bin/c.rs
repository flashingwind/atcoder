use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: i64,
    };
    let mut cache = FxHashMap::default();
    if let Some((s, _)) = bin(n, "".to_string(), 0, &mut cache) {
        if s.len() == 0 {
            println!("0");
        }
        println!("{}", s.chars().rev().collect::<String>());
    } else {
        println!("0");
    }
}
fn bin(n: i64, s: String, t: u32, cache: &mut FxHashMap<String, i64>) -> Option<(String, i64)> {
    let v = *cache.entry(s.clone()).or_insert(0);
    // println!("{s}: {v}");
    if n == v {
        return Some((s, v));
    } else if n.abs() < v.abs() || t == 30 {
        // println!("break");
        return None;
    }
    let tt = t + 1;
    if tt % 2 == 0 {
        *cache.entry(s.clone() + "1").or_insert(0) = v - 2u64.pow(t) as i64;
        // println!("    -{}", 2u64.pow(tt) as i64);
    } else {
        *cache.entry(s.clone() + "1").or_insert(0) = v + 2i64.pow(t) as i64;
        // println!("    +{}", 2u64.pow(tt) as i64);
    }
    if let Some(ret) = bin(n, s.clone() + "1", tt, cache) {
        return Some(ret);
    } else {
        *cache.entry(s.clone() + "0").or_insert(0) = v;
        if let Some(ret) = bin(n, s.clone() + "0", tt, cache) {
            return Some(ret);
        } else {
            return None;
        }
    }
}
