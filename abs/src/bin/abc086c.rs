use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut route = vec![(0usize, 0i64, 0i64)];
    for _ in 0..n {
        input! {
            t: usize,
            x: i64,
            y: i64,
        }
        route.push((t, x, y));
    }
    route.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));
    route.reverse();
    if check(&mut route) {
        println!("Yes")
    } else {
        println!("No");
    }
}

fn check(route: &mut Vec<(usize, i64, i64)>) -> bool {
    while 2 <= route.len() {
        let (t1, x1, y1) = route.pop().unwrap();
        let (t2, x2, y2) = *route.last().unwrap();
        println!("{}: ({},{})->{}:({},{})", t1, x1, y1, t2, x2, y2);
        let d = (x1 - x2).abs() + (y1 - y2).abs();
        let dt = (t2 - t1) as i64;
        if !(d == dt || d < dt && (dt - d) % 2 == 0) {
            return false;
        };
        if route.len() == 0 {
            return false;
        }
    }
    return false;
}
