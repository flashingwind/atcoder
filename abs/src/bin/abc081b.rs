use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut arr = Vec::new();
    (0..n).for_each(|_: usize| {
        input! {
            ainp: i32,
        }
        arr.push(ainp);
    });
    let mut is_to_be_broken = false;
    for t in 0..(1 + 10i32.pow(9)) {
        for a in arr.iter_mut() {
            if (*a % 2) == 0 {
                *a /= 2;
                //println!("t={} div {}/2", t, *a);
            } else {
                //println!("break t={}", t);
                println!("{}", t);
                is_to_be_broken = true;
                break;
            }
        }
        if is_to_be_broken {
            break;
        }
    }
}
