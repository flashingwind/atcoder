use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a_arr: [u32;n],
        mut b_arr: [u32;m],
    };
    a_arr.sort_unstable();
    b_arr.sort_unstable();

    //println!("a_arr={:?}", a_arr);
    //println!("b_arr={:?}", b_arr);
    let mut min = std::u32::MAX;
    let mut i = 0;
    let mut j = 0;
    for _t in 0..n + m - 1 {
        // println!(
        //     "a{}:{} - b{}:{} = {}",
        //     i,
        //     a_arr[i],
        //     j,
        //     b_arr[j],
        //     abs_diff(a_arr[i], b_arr[j])
        // );
        if abs_diff(a_arr[i], b_arr[j]) < min {
            min = abs_diff(a_arr[i], b_arr[j]);
        }
        match a_arr[i].cmp(&b_arr[j]) {
            Ordering::Less => {
                i += 1;
            }
            Ordering::Greater => {
                j += 1;
            }
            Ordering::Equal => {
                i += 1;
                j += 1;
            }
        };
        if n <= i || m <= j {
            break;
        }
    }
    println!("{}", min);
}

fn abs_diff(a: u32, b: u32) -> u32 {
    if a < b {
        b - a
    } else {
        a - b
    }
}
