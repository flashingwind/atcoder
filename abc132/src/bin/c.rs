use proconio::input;

fn main() {
    input! {
        n: usize,
        mut d: [u32;n],
    };
    d.sort_unstable();
    // println!("{:?}", d);
    let lower_th = d[n / 2 - 1];
    let upper_th = d[n / 2];
    if lower_th <= upper_th {
        // println!("upper_th={upper_th} lower_th={lower_th}");
        println!("{}", upper_th - lower_th);
    } else {
        println!("0");
    }
}
