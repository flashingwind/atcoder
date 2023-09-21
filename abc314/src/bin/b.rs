use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut b = Vec::new();
    for _ in 0..n {
        input! {
            c: usize,
            a: [usize;c],
        };
        b.push(a);
    }
    input! {
        x: usize,
    };
    let mut len_min = usize::MAX;
    for bb in b.iter() {
        if bb.len() < len_min && bb.contains(&x) {
            len_min = bb.len();
        }
    }
    let mut len_min_i = Vec::new();
    for (i, bb) in b.iter().enumerate() {
        if bb.len() == len_min && bb.contains(&x) {
            len_min_i.push(i + 1);
        }
    }
    len_min_i.sort();
    println!("{}", len_min_i.len());
    println!(
        "{}",
        len_min_i
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
