use std::collections::{btree_map::Entry, BTreeMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        poems: [(String,u32);n],
    };
    let mut org: BTreeMap<String, u32> = BTreeMap::new();
    let mut max = 0;
    let mut max_i = 0;
    for (i, p) in poems.iter().enumerate() {
        match org.entry(p.0.to_owned()) {
            Entry::Occupied(_) => (),
            Entry::Vacant(ent) => {
                if max < p.1 {
                    max = p.1;
                    max_i = i;
                }
                ent.insert(p.1);
            }
        };
    }
    println!("{}", max_i + 1);
}
