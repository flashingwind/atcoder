use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u32,
        arr: [u32;n],
    };
    let mut cost = 0;
    for (i, a) in arr.iter().enumerate() {
        if i % 2 == 1 {
            cost += *a - 1;
        } else {
            cost += *a;
        }
        if x < cost {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
