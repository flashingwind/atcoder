use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64;n],
        mut x: u64,
    };
    let sum: u64 = a.iter().sum();
    let mut cnt = (x / sum) * n as u64;
    x = x % sum;
    // println!("cnt={cnt} sum={sum} x={x}");
    for (i, _) in a.iter().enumerate() {
        if x == 0 {
            cnt += 1;
            break;
        } else if x < a[i] {
            cnt += 1;
            break;
        } else {
            cnt += 1;
            x -= a[i];
        }
    }
    // println!("cnt={cnt} x={x}");
    println!("{}", cnt);
}
