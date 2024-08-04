use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize;n],
    };
    if n == 0 {
        println!("1");
    }
    a.reverse();
    let mut car = 1;
    let mut cnt = 0usize;
    loop {
        if let Some(aa) = a.pop() {
            if car + aa <= k {
                car += aa;
                // println!("{cnt}:car={car} {}<-{:?}", aa, a);
            } else {
                cnt += 1;
                a.push(aa);
                // println!("{cnt}:car={car} {:?}", a);
                car = 0;
            }
        }
        if a.is_empty() {
            // println!("{:?}", a);
            break;
        }
    }
    println!("{}", cnt + 1);
}
