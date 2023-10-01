use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };

    let step = if c < d { d } else { c };
    let div = if c < d { c } else { d };
    let mut cnt = 0;
    for n in (a..=b).step_by(step) {
        cnt += step - (step / div) - 1;
        if n % div == 0 {
            cnt -= 1;
        }
    }
    println!("{}", cnt);
}
