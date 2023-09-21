use std::{collections::HashMap, vec};

use proconio::input;

fn check_r(
    p1: (usize, i32, i32),
    p2: (usize, i32, i32),
    mut is_checked: &mut HashMap<(usize, i32, i32), &mut HashMap<(usize, i32, i32), bool>>,
) -> bool {
    let (t1, x1, y1) = p1;
    let (t2, x2, y2) = p2;
    println!("{}: ({},{})->{}:({},{})", t1, x1, y1, t2, x2, y2);
    is_checked
        .entry(p1)
        .or_insert(&mut HashMap::new() as &mut HashMap<(usize, i32, i32), bool>);
    let &mut is_chk = match is_checked.get_mut(&p1) {
        Some(f2) => match f2.get_mut(&p2) {
            Some(ff2) => ff2,
            None => {
                let mut b = false;
                f2.insert(p2, b);
                &mut b.clone()
            }
        },
        None => {
            let b = false;
            is_checked.insert(p1, &mut HashMap::new());
            return false;
        }
    };
    if is_chk {
        return false;
    }
    if t2 < t1 {
        is_chk = true;
        return false;
    } else if t1 == t2 {
        if x1 == x2 && y1 == y2 {
            return true;
        } else {
            is_chk = true;
            return false;
        }
    }
    if check_r((t1 + 1, x1 + 1, y1), (t2, x2, y2), &mut is_checked) {
        return true;
    }
    if check_r((t1 + 1, x1, y1 + 1), (t2, x2, y2), &mut is_checked) {
        return true;
    }
    if check_r((t1 + 1, x1 - 1, y1), (t2, x2, y2), &mut is_checked) {
        return true;
    }
    if check_r((t1 + 1, x1, y1 - 1), (t2, x2, y2), &mut is_checked) {
        return true;
    }
    is_chk = true;
    return false;
}

fn check(route: &Vec<(usize, i32, i32)>) -> bool {
    let mut is_checked: HashMap<(usize, i32, i32), &mut HashMap<(usize, i32, i32), bool>> =
        HashMap::new();
    return check_r(route[0], route[route.len() - 1], &mut is_checked);
}

fn main() {
    input! {
        n: i32,
    }
    let mut route: Vec<(usize, i32, i32)> = vec![(0, 0, 0)];
    for _ in 0..n {
        input! {
            t: usize,
            x: i32,
            y: i32,
        }
        route.push((t, x, y));
    }
    route.sort_by(|(a, _, _), (b, _, _)| a.partial_cmp(b).unwrap());
    if check(&route) {
        println!("Yes")
    } else {
        println!("No");
    }
}
