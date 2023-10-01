use proconio::input;

fn main() {
    input! {
        n: i64,
        l: i64,
    };
    let mut min = i64::MAX;
    let mut aji = vec![];
    for i in 1..=n {
        aji.push(l + i - 1);
        if aji.last().unwrap().abs() < min {
            min = aji.last().unwrap().abs();
        }
    }
    let sum: i64 = aji.iter().filter(|&&v| v.abs() != min).sum();
    println!("{}", sum);
}
