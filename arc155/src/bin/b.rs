use proconio::input;

fn main() {
    input! {
        num_q: usize,
        a1: u64,
        b1: u64,
        q_vec: [(u32,u64,u64);num_q],
    };
    let mut t_vec = vec![(a1, b1)];
    for (op, a, b) in q_vec.iter() {
        if *op == 1 {
            t_vec.push((*a, *b));
            // println!("T={:?}", t_vec);
        } else if *op == 2 {
            // println!("T={:?}", t_vec);
            // println!("min a={a} b={b}");
            println!("{}", min_fs(*a, *b, &t_vec));
        }
    }
}
fn min_fs(a: u64, b: u64, t_vec: &Vec<(u64, u64)>) -> u64 {
    let mut min_fs = std::u64::MAX;
    let mut start = a;
    let mut end = b;

    for (a_t, b_t) in t_vec {
        if a < *a_t {
            start = a;
        } else {
            start = *a_t;
        }
        if b < *b_t {
            end = b;
        } else {
            end = *b_t;
        }
        for x in start..=end {
            let tmpfs = (x.abs_diff(*a_t)).abs_diff(*b_t);
            //println!("x={x} tmpfs={tmpfs}");
            if tmpfs < min_fs {
                min_fs = tmpfs;
            }
        }
    }
    min_fs
}
// fn fs(x: u64, s: &Vec<(u64, u64)>) -> u64 {
//     let mut min_diff = std::u64::MAX;
//     let mut b_min_diff = std::u64::MAX;
//     let mut a_min_diff = std::u64::MAX;
//     for (a, b) in s {
//         let b_tmp: u64 = if *a <= x { x - *a } else { *a - x };
//         if b_tmp.abs_diff(*b) < min_diff {
//             min_diff = b_tmp.abs_diff(*b);
//             b_min_diff = *b;
//             a_min_diff = *a;
//         }
//     }
//     x.abs_diff(a_min_diff).abs_diff(b_min_diff)
// }
// fn fs_org(x: u64, s: Vec<(u64, u64)>) -> u64 {
//     let mut min = std::u64::MAX;
//     for (a, b) in s {
//         let tmp = (x).abs_diff(a).abs_diff(b);
//         if tmp < min {
//             min = tmp;
//         }
//     }
//     min
// }
