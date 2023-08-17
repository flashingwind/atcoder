use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [usize;n],
    };
    // count population
    let mut x_max = 100;
    let mut cell = vec![0usize; x_max + 1];
    x_max = 0;
    for (_i, x_i) in x.iter().enumerate() {
        cell[*x_i] += 1;
        if x_max < *x_i {
            x_max = *x_i;
        }
    }
    cell = cell.get(1..=x_max).unwrap().to_vec();
    cell.insert(0, 0);
    // println!("cell={:?}", cell);
    // cost
    let mut cost = vec![0; x_max + 1];
    for (p, c) in cost.iter_mut().enumerate().skip(1) {
        for (i, pop) in cell.iter_mut().enumerate().skip(1) {
            if p <= i {
                *c += (i - p).pow(2) * *pop;
            } else {
                *c += (p - i).pow(2) * *pop;
            }
            // println!("{p},{i}: c+=({}-{})^2*{}", i, p, *pop);
        }
        // println!("{p}: c={}", *c);
    }
    let min = *cost.iter().skip(1).min().unwrap();
    println!("{}", min);
}
