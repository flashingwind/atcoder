// use std::collections::BTreeMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize;n],
    };
    // let mut tup = BTreeMap::new();
    let mut tup = vec![usize::MAX; n + 1];
    for (i, &ai) in a.iter().enumerate() {
        // tup.entry(ai).or_insert((i + 1) as isize);
        if ai != -1 {
            tup[ai as usize] = i + 1;
        } else {
            tup[0] = i + 1;
        }
        // println!("{}->{:?}", i, (ai, (i + 1) as isize));
    }
    // tup.sort();
    // println!("{:?}", tup);
    // let mut i = *tup.entry(-1).or_insert(0isize);
    let mut i = tup[0];
    let mut out = vec![];
    out.push(i);
    loop {
        // println!("{}->{:?}", i, *tup.entry(i).or_insert(0));
        // i = *tup.entry(i).or_insert(0);
        i = tup[i];
        out.push(i);
        if out.len() == n {
            break;
        }
    }

    println!(
        "{}",
        out.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
