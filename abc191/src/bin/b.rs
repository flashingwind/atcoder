use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u32,
        a: [u32;n],
    };
    let mut a2 = vec![];
    for aa in a.iter() {
        if *aa != x {
            a2.push(aa.to_string());
        }
    }
    println!("{}", a2.join(" "));
}
