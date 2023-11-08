use proconio::input;

fn main() {
    let mut xs = vec![];
    let mut ys = vec![];
    let mut ps = vec![];
    for _ in 0..3 {
        input! {
            x: i32,
            y: i32,
        };
        if !xs.contains(&x) {
            xs.push(x);
        }
        if !ys.contains(&y) {
            ys.push(y);
        }
        ps.push((x, y));
    }
    // println!("{:?}", xs);
    // println!("{:?}", ys);
    // println!("{:?}", ps);
    for &x in xs.iter() {
        for &y in ys.iter() {
            if !ps.contains(&(x, y)) {
                println!("{x} {y}");
                return;
            }
        }
    }
}
