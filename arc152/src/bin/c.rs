use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [i64;n],
    };
    loop {
        //println!("1: aa={:?}", aa);
        let mut a_max = aa[0];
        // a_max
        for a in aa.iter() {
            if *a > a_max {
                a_max = *a;
            }
        }
        // check if s exists, that meet the conditions a_max<2s
        // a_i' := 2s-a_i >= 0
        let mut is_changed = false;
        let mut s = aa[0];
        for a in aa.iter() {
            if a_max >= 2 * *a {
                s = *a;
                is_changed = true;
                //println!("s:  a_max={} <= 2*a={} s={}", a_max, 2 * *a, s);
            }
        }
        if is_changed {
            // calc a_i, search a_max
            //let mut
            a_max = aa[0];
            for a in aa.iter_mut() {
                //println!("calc a_i: s={s}, a_i=2*s-a_i=2*{}-{}={}", s, *a, 2 * s - *a);
                *a = 2 * s - *a;
                if a_max < *a {
                    a_max = *a;
                }
            }
            //println!("2: aa={:?}", aa);
        }else{
            println!("{}", a_max);
            return;
        }
        //break;
    }
}
