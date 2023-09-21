use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i32,
        p: [(i32,i32);n],
    };
    // p.sort();
    // let mut pair = vec![vec![false; n]; n];
    let mut pair = Vec::with_capacity(n * n);
    let mut is_i = vec![false; n];
    is_i[0] = true;
    for i in 0..n {
        for j in i + 1..n {
            let tmp = ((p[i].0 - p[j].0) * (p[i].0 - p[j].0)
                + (p[i].1 - p[j].1) * (p[i].1 - p[j].1))
                <= (d * d);
            // println!(
            //     "{i}: {j}: ({},{})^2+({},{})^2 {}",
            //     p[i].0,
            //     p[j].0,
            //     p[i].1,
            //     p[j].1,
            //     (p[i].0 - p[j].0) * (p[i].0 - p[j].0) + (p[i].1 - p[j].1) * (p[i].1 - p[j].1)
            // );
            if tmp {
                pair.push((i, j));
                pair.push((j, i));
            }
            // pair[j][i] = tmp;
        }
    }
    // println!("{:?}", pair);
    let mut is_changed = true;
    while is_changed {
        is_changed = false;
        for &(i, j) in pair.iter() {
            if is_i[i] && !is_i[j] {
                is_i[j] = true;
                is_changed = true;
                // println!("{i}->{j}");
            }
        }
        // println!("{:?}", is_i);
    }
    for f in is_i.iter() {
        println!("{}", if *f { "Yes" } else { "No" });
    }
}
