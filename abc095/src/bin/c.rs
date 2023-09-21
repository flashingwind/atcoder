use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        mut x: u32,
        mut y: u32,
    };
    let mut cost = 0;
    let n;
    if x < y {
        n = x;
    } else {
        n = y;
    }
    x -= n;
    y -= n;
    if a + b <= c * 2 {
        cost += a * n + b * n;
        // println!("A*{} B*{}", n, n);
    } else {
        cost += 2 * c * n;
        // println!("AB*{n}");
    }
    if a <= 2 * c {
        cost += x * a;
        // println!("A*{}", x);
    } else {
        cost += x * 2 * c;
        // println!("AB*{}", x);
    }
    if b <= 2 * c {
        cost += y * b;
        // println!("B*{}", y);
    } else {
        cost += y * 2 * c;
        // println!("AB*{}", y);
    }

    println!("{}", cost);
}
