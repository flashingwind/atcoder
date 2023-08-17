use std::collections::BTreeMap;

use proconio::input;

fn main() {
    #[allow(non_snake_case)]
    let N: usize = 2usize.pow(20);
    input! {
        num_q: usize,
        q: [(u32,usize);num_q],
    };
    //let mut arr = vec![std::usize::MAX; N];
    let mut arr = BTreeMap::new();
    // let mut i_of_filled = Vec::new();
    for (t, x) in q.iter() {
        if *t == 2 {
            let h = *x % N;
            // println!("print a{h}<={}", arr[h]);
            if let Some(a) = arr.get(&h) {
                println!("{}", a);
            } else {
                println!("-1");
            }
        } else if *t == 1 {
            let h = *x % N;
            let mut is_changed = false;
            {
                let arr2 = arr.to_owned();
                let mut itr = arr2.iter().filter(|(&k, &_)| h <= k).peekable();
                #[allow(clippy::unnecessary_to_owned)]
                for (hh, x) in itr.to_owned() {
                    let hhh_max = *itr.peek().unwrap_or(&(&(N - 1), &std::usize::MAX)).0;
                    for hhh in *hh..hhh_max {
                        println!("a{}={}", hh, *x);
                        if arr.get(&hhh).is_none() {
                            arr.insert(hhh, *x);
                            is_changed = true;
                            break;
                        }
                    }
                }
            }
            if !is_changed {
                let arr2 = arr.to_owned();
                let mut itr = arr2.iter().filter(|(&k, &_)| k < h).peekable();
                let hhh_max = *itr.peek().unwrap_or(&(&(N - 1), &std::usize::MAX)).0;

                for hhh in 0..hhh_max {
                    println!("a{}={}", hhh, *x);
                    if arr.get(&hhh).is_none() {
                        arr.insert(hhh, *x);
                        break;
                    }
                }
            }
        }
        println!("{:?}", arr);
    }
}
