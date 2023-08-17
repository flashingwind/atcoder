use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1;n],
    };
    let mut i = 0;
    for _ in 0..n {
        i = a[i];
    }
    let mut b = vec![i];
    while a[i] != b[0] {
        i = a[i];
        b.push(i);
    }
    println!("{}", b.len());
    println!(
        "{}",
        b.iter()
            .map(|v| (v + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
