use proconio::input;

fn main() {
    input! {
        n: usize,
        edges: [(usize,usize);n-1]
    };

    let mut is_eq_0 = true;
    for e in edges.to_owned().iter() {
        if !(e.0 == edges[0].0 || e.1 == edges[0].0) {
            is_eq_0 = false;
            // println!("{}!= {} {}", edges[0].0, e.0, e.1);
            break;
        }
        // println!("{}== {} {}", edges[0].0, e.0, e.1);
    }
    let mut is_eq_1 = true;
    for e in edges.to_owned().iter() {
        if !(e.0 == edges[0].1 || e.1 == edges[0].1) {
            is_eq_1 = false;
            // println!("{}!= {} {}", edges[0].1, e.0, e.1);
            break;
        }
        // println!("{}== {} {}", edges[0].1, e.0, e.1);
    }
    if is_eq_0 || is_eq_1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
