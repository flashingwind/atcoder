use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut a: [(Usize1,Usize1);n],
    };
    let nn = 2 * n;
    a.sort();
    let mut gen = vec![0; nn];
    for i in 0..n {
        if a[i].0 % nn < a[i].1 % nn {
            gen[a[i].0 % nn] += 1;
            gen[a[i].1 % nn] -= 1;
        } else {
            gen[a[i].1 % nn] += 1;
            gen[a[i].0 % nn] -= 1;
        }
        println!("dif{},{}:{:?}", a[i].0 % nn, a[i].1 % nn, gen);
    }
    for i in 1..nn {
        gen[i] += gen[i - 1];
    }
    println!("sum{:?}", gen);
    for i in 1..nn {
        if 2 <= gen[i] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
