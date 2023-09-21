use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n_obs: usize,
        n_way: usize,
        mut h: [i32; n_obs],
    };
    h.insert(0, 0);
    let mut ways = HashMap::new();
    for _ in 1..=n_way {
        input! {
            mut a: usize,
            mut b: usize,
        };
        ways.entry(a).or_insert(Vec::new() as Vec<usize>);
        if let Some(w) = ways.get_mut(&a) {
            w.push(b);
        }
        ways.entry(b).or_insert(Vec::new() as Vec<usize>);
        if let Some(w) = ways.get_mut(&b) {
            w.push(a);
        }
    }
    //check all
    let mut cnt = 0;
    for i1 in 1..=n_obs {
        let mut is_good = true;
        //println!("{}->: h1={}", i1, h[i1]);
        if let Some(w) = ways.get_mut(&i1) {
            for i2 in w.iter() {
                //println!("   {}: h2={}", i2, h[*i2]);
                if h[i1] <= h[*i2] {
                    //println!("    not good; break");
                    is_good = false;
                    break;
                }
            }
        }
        //println!("    i1={} good={}", i1, is_good);
        if is_good {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
