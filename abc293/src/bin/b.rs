use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
    };
    a.insert(0, 0);
    let mut is_called = vec![false; n + 1];
    is_called[0] = true;
    for (i, ai) in a.iter().enumerate().skip(1) {
        if !is_called[i] {
            // print!("{i}:{ai} ");
            is_called[*ai] = true;
        }
    }
    let mut cnt = 0;
    // println!("{:?}", is_called);
    for (_, b) in is_called.iter().enumerate().skip(1) {
        if !*b {
            cnt += 1;
        }
    }
    println!("{}", cnt);
    let mut is_first = true;
    for (i, b) in is_called.iter().enumerate().skip(1) {
        if !*b && is_first {
            print!("{}", i);
            is_first = false;
        } else if !*b {
            print!(" {}", i);
        }
    }
    println!();
}
