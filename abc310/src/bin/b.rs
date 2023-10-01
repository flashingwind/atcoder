use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
    };
    let mut fs = vec![vec![0u32; 0]; n];
    for i in 0..n {
        input! {
            p: u32,
            c: usize,
            f: [u32;c]
        };
        fs[i] = f;
        fs[i].insert(0, p);
    }
    fs.sort_by(|a, b| a[0].cmp(&b[0]));
    // fs.reverse();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            // println!("{i}:{:?} {j}:{:?}", fs[i], fs[j]);
            if fs[j][0] <= fs[i][0] {
                let mut is_ok = true;
                for f in fs[i].iter().skip(1) {
                    // print!("  F1: f{f} is not included in {i}:{:?} ", fs[j]);
                    if !(fs[j][1..].contains(f)) {
                        // println!(": true");
                        is_ok = false;
                        break;
                    } else {
                        // println!(": false");
                    }
                }
                if is_ok {
                    let mut is_fj_over_fi = false;
                    for f in fs[j].iter().skip(1) {
                        if !(fs[i][1..].contains(f)) {
                            // println!("  F2: f{f} is not included in {j} ");
                            is_fj_over_fi = true;
                            break;
                        }
                    }
                    if fs[j][0] < fs[i][0] || is_fj_over_fi {
                        // println!("{}&&({}||{})", is_ok, fs[j][0] < fs[i][0], is_fj_over_fi);
                        println!("Yes");
                        return;
                    }
                }
            } else {
                break;
            }
        }
    }
    println!("No");
}
