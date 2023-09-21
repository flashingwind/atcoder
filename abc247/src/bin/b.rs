use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize,
        st: [(String,String);n],
    };
    let mut map: FxHashMap<String, _> = FxHashMap::default();

    for (i, (s, t)) in st.iter().enumerate() {
        map.entry(s.to_owned()).or_insert(Vec::new()).push(('s', i));
        if *s != *t {
            map.entry(t.to_owned()).or_insert(Vec::new()).push(('t', i));
        }
    }
    for (s, t) in st.iter() {
        let len_s = map.entry(s.to_owned()).or_insert(Vec::new()).len();
        let len_t = map.entry(t.to_owned()).or_insert(Vec::new()).len();
        if len_s != 1 && len_t != 1 {
            // println!(
            //     "{},{}",
            //     map.entry(s.to_owned()).or_insert(Vec::new()).len(),
            //     map.entry(t.to_owned()).or_insert(Vec::new()).len()
            // );
            println!("No");
            return;
        }
    }
    println!("Yes");
}
