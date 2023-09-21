use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let mut map = HashMap::new();
    map.insert(1, (0, 0));
    map.insert(2, (0, 1));
    map.insert(3, (0, 2));
    map.insert(4, (1, 0));
    map.insert(5, (1, 1));
    map.insert(6, (1, 2));
    map.insert(7, (2, 0));
    map.insert(8, (2, 1));
    map.insert(9, (2, 2));
    if let Some(&(xa, ya)) = map.get(&a) {
        if let Some(&(xb, yb)) = map.get(&b) {
            if xa == xb && ya + 1 == yb {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
