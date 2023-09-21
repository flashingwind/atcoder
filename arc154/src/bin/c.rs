use proconio::input;

fn main() {
    input! {
        tests: usize,
    };
    for _t in 1..=tests {
        prc_test();
    }
}

fn prc_test() {
    input! {
        n: usize,
        a: [u64;n],
        mut b: [u64;n],
    };

    let mut is_eq = true;
    for i in 0..n {
        let tmp_a = -1;
        let tmp_b = -1;
        for j in i..n {
            let i = i_org % n;
            if a[i] != b[i] {
                {
                    println!(" !=   a={:?}", a);
                    let j = (i + 1) % n;
                    if a[j] == b[i] {
                        a[i] = a[j];
                        is_eq = is_eq && true;
                        println!(" cpied a={:?}", a);
                    } else {
                        is_eq = false;
                    }
                }
            }
            b.rotate_left(1);
        }
        if is_eq {
            break;
        }
    }
    // println!("a{:?}", a);
    // println!("b{:?}", b);
    if is_eq {
        println!("Yes");
    } else {
        println!("No");
    }
}
