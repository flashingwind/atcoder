use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };
    let mut op = Vec::new();
    while 0 < n {
        if n % 2 == 0 {
            n /= 2;
            op.push("B");
            // println!("B: {}", n);
        } else {
            n -= 1;
            op.push("A");
            // println!("A {}:", n);
        }
    }
    for o in op.iter().rev() {
        print!("{}", o);
    }
    println!();
}
