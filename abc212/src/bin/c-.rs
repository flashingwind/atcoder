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
    for a in a_arr.iter() {
        for b in b_arr.iter() {
            if abs_diff(*a, *b) < min {
                min = abs_diff(*a, *b);
                //println!("{}-{}", *a, *b);
                if *a < *b {
                    break;
                }
            }
        }
        if *b_arr.last().unwrap() < *a {
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
