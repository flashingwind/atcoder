use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64;n],
    };
    let mut even_a_max = 0;
    let mut even_a_max2 = 0;
    let mut odd_a_max = 0;
    let mut odd_a_max2 = 0;
    let mut cnt_odd = 0;
    let mut cnt_even = 0;
    for ai in a.iter() {
        if *ai % 2 == 0 {
            if even_a_max <= *ai {
                even_a_max2 = even_a_max;
                even_a_max = *ai;
                cnt_even += 1;
            } else if even_a_max2 <= *ai {
                even_a_max2 = *ai;
                cnt_even += 1;
            }
        } else {
            if odd_a_max <= *ai {
                odd_a_max2 = odd_a_max;
                odd_a_max = *ai;
                cnt_odd += 1;
            } else if odd_a_max2 <= *ai {
                odd_a_max2 = *ai;
                cnt_odd += 1;
            }
        }
    }
    let even_sum = even_a_max + even_a_max2;
    let odd_sum = odd_a_max + odd_a_max2;
    // println!("odd_sum={odd_sum} even_sum={even_sum}");

    if 2 <= cnt_even && 2 <= cnt_odd {
        if odd_sum < even_sum {
            println!("{}", even_sum);
        } else {
            println!("{}", odd_sum);
        }
    } else if 2 <= cnt_odd {
        println!("{}", odd_sum);
    } else if 2 <= cnt_even {
        println!("{}", even_sum);
    } else {
        println!("-1");
    }
}
