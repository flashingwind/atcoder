use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: isize,
        arr: [Chars;n as usize],
    };
    let mut a_max = 0;
    for (_, l) in arr.iter().enumerate() {
        for (_, &a) in l.iter().enumerate() {
            if a_max < a.to_digit(10).unwrap() as u64 {
                a_max = a.to_digit(10).unwrap() as u64;
            }
        }
    }
    let mut sum_max: u64 = 0;
    for (k, line) in arr.iter().enumerate() {
        for (l, a) in line.iter().enumerate() {
            // println!(
            //     "{},{}",
            //     k,l
            // );
            let patts: Vec<(isize, isize)> = vec![
                (0, 1),
                (0, -1),
                (1, 0),
                (1, 1),
                (1, -1),
                (-1, 0),
                (-1, 1),
                (-1, -1),
            ];
            if a_max == a.to_digit(10).unwrap() as u64 {
                for (di, dj) in patts {
                    let mut sum: u64 = 0;
                    // println!("pat={di},{dj}");
                    let mut i = k as isize;
                    let mut j = l as isize;
                    for _ in 0..=(n - 1) {
                        sum *= 10;
                        // println!("{i},{j}={}", arr[i as usize][j as usize]);
                        sum += arr[i as usize][j as usize].to_digit(10).unwrap() as u64;
                        i += di;
                        j += dj;
                        if i < 0 {
                            i += n;
                        } else if n <= i {
                            i %= n;
                        }
                        if j < 0 {
                            j += n;
                        } else if n <= j {
                            j %= n;
                        }
                    }
                    if sum_max < sum {
                        sum_max = sum;
                    }
                }
            }
        }
    }
    println!("{}", sum_max);
}
