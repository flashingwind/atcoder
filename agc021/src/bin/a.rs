use proconio::input;

fn main() {
    input! {
        mut n: u64,
    };
    let mut max = sum_digit(n);
    let lower = calc_lower_bond(n);
    // println!("max(init.)={max} lower={lower}");

    if n == 0 {
        println!("0");
        return;
    }

    for m in (0..=n).rev() {
        // println!("m={m}");
        let sum = sum_digit(m);
        if max < sum {
            max = sum;
            // println!("max={}", max);
        } else if m <= lower {
            // println!("lower");
            break;
        }
    }
    // println!("max={}", max);
    println!("{}", max);
}

fn sum_digit(mut n: u64) -> u64 {
    let mut sum = 0u64;
    while 0 < n {
        //// println!("{}", n % 10u64);
        sum += n % 10u64;
        n /= 10u64;
    }
    //// println!("{}", sum);
    return sum;
}

fn calc_lower_bond(n: u64) -> u64 {
    /*
    let mut lower = 0;

    let mut d = vec![0u64; 0];
    while 0 < n {
        if (n % 10) <= 0 {
            d.push(n % 10);
        } else {
            d.push(n % 10 - 1);
        }
        n /= 10;
    }
    for dd in d.iter() {
        lower *= 10;
        lower += dd;
    }
    // println!("lower={lower}");
    return lower;
    */
    let n_str = format!("{}", n).to_string();
    let len = n_str.len() - 1;
    // println!("n={} >= 10u64.pow(len as u32)={}", n, 10u64.pow(len as u32));
    let mut sum: u64 = n_str.get(0..=0).unwrap().parse::<u64>().unwrap() - 1;
    for _ in 0..len {
        sum *= 10;
        sum += 9;
    }
    return sum;
}
