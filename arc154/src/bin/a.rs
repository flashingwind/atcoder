use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a_c: Chars,
        b_c: Chars,
    };
    let mut a_vec = to_i_vec(&a_c);
    let mut b_vec = to_i_vec(&b_c);
    let mut a: i128 = sum(&a_vec);
    let mut b: i128 = sum(&b_vec);

    for i in 0..n {
        // println!("{}:tr  a={} b={}", i, a_vec[i], b_vec[i]);
        match swap(&a_vec, &b_vec, a, b, i, n) {
            Some((a_tr, b_tr)) => {
                let tmp = a_vec[i];
                a_vec[i] = b_vec[i];
                b_vec[i] = tmp;
                a = a_tr;
                b = b_tr;
                // println!(" some a_tr={} b_tr={} a*b={}", a_tr, b_tr, a_tr * b_tr);
            }
            None => (),
        }

        println!(
            "a {}=={}",
            a.to_string(),
            a_vec.iter().map(|x| x.to_string()).collect::<String>()
        );
        println!(
            "b {}=={}",
            b.to_string(),
            b_vec.iter().map(|x| x.to_string()).collect::<String>()
        );
    }
    println!("{}", (a * b) % 998244353i128);
}

fn to_i_vec(a: &Vec<char>) -> Vec<i128> {
    let mut v = vec![0i128; 0];
    for (i, c) in a.iter().enumerate() {
        v.insert(i, c.to_digit(10).unwrap() as i128);
    }
    return v;
}

fn sum(a: &Vec<i128>) -> i128 {
    let mut a_mult = 0i128;
    for aa in a.iter() {
        a_mult *= 10;
        a_mult += *aa;
    }
    return a_mult;
}

fn swap(
    a_vec: &Vec<i128>,
    b_vec: &Vec<i128>,
    a: i128,
    b: i128,
    i: usize,
    n: usize,
) -> Option<(i128, i128)> {
    // println!("swap a={} b={} a*b={}", a, b, a * b);
    let diff_a = a_vec[i] * 10_i128.pow(n as u32 - i as u32 - 1);
    let diff_b = b_vec[i] * 10_i128.pow(n as u32 - i as u32 - 1);
    // println!("swap diff_a={} diff_b={}", diff_a, diff_b);
    let tmp_a = a - diff_a + diff_b;
    let tmp_b = b - diff_b + diff_a;

    // println!("  tmp a={} b={} a*b={}", tmp_a, tmp_b, tmp_a * tmp_b);
    // if a * b > tmp_a * tmp_b {
    if a_vec[i] < b_vec[i] {
        // println!("  swaped a={} b={} a*b={}", tmp_a, tmp_b, tmp_a * tmp_b);
        return Some((tmp_a, tmp_b));
    }
    return None;
}
