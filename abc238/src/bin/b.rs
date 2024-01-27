use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64;n],
    };
    let mut cuts = vec![0];
    for aa in a.iter() {
        let b = cuts.last().unwrap();
        cuts.push((b + aa) % 360);
    }
    cuts.sort();
    let mut rad = vec![];
    let mut c_last = cuts[0];
    for i in 1..cuts.len() {
        // println!("rad cuts[{i}]:{}-c_last:{}", cuts[i], c_last);
        rad.push(cuts[i] - c_last);
        c_last = cuts[i];
    }
    rad.push(360 - c_last);
    rad.sort();
    println!("{}", rad.last().unwrap());
}
