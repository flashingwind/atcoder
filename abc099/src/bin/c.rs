use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };
    let mut pow9 = vec![9];
    let mut pow6 = vec![6];

    loop {
        if let Some(&pow9_last) = pow9.last() {
            // println!("pow9_last={pow9_last}");
            if n <= pow9_last {
                break;
            }
            pow9.push(pow9_last * 9);
        } else {
            break;
        }
    }
    loop {
        if let Some(&pow6_last) = pow6.last() {
            // println!("pow6_last={pow6_last}");
            if n <= pow6_last {
                break;
            }
            pow6.push(pow6_last * 6);
        } else {
            break;
        }
    }
    let mut pow: Vec<_> = pow9.iter().filter(|v| **v != 0).map(|v| *v).collect();
    pow.extend(pow6.iter().filter(|v| **v != 0));
    pow.push(1);
    pow.sort();
    // println!("{:?}", pow9);
    // println!("{:?}", pow6);
    // println!("{:?}", pow);
    let mut cnt = 0;

    // 全探索にしないとだめっぽい
    for &p in pow.iter().rev() {
        if 0 < n / p {
            // print!("n={n}-{}*{p}", n / p);
            cnt += n / p;
            n %= p;
            // println!("={n} p={p}");
        }
    }

    println!("{}", cnt);
}
