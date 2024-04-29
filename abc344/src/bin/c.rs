use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32;n],
        m: usize,
        mut b: [u32;m],
        l: usize,
        mut c: [u32;l],
        q: usize,
        x: [u32;q],
    };
    a.sort();
    b.sort();
    c.sort();
    for xx in x.iter() {
        println!("X={}:", xx);
        let mut is_broken = false;
        for aa in a.iter() {
            // println!("  a={aa}: a={}", *aa);
            if *xx < *aa {
                println!("No");
                break;
            }
            for bb in b.iter() {
                // println!("    b={bb}: a+b={}", *aa + *bb);
                if *xx < *aa + *bb {
                    println!("No");
                    break;
                }
                for cc in c.iter() {
                    if *xx < *aa + *bb + *cc {
                        // println!(
                        //     "      c={cc}: a+b+c={}: {}=={}",
                        //     *aa + *bb + *cc,
                        //     *aa + *bb + *cc,
                        //     *xx
                        // );
                        println!("No");
                        break;
                    } else if *aa + *bb + *cc == *xx {
                        // println!(
                        //     "      c={cc}: a+b+c={}: {}=={}",
                        //     *aa + *bb + *cc,
                        //     *aa + *bb + *cc,
                        //     *xx
                        // );
                        print!("Yes");
                        is_broken = true;
                        break;
                    }
                }
                if is_broken {
                    break;
                }
            }
            if is_broken {
                break;
            }
        }
    }
}
