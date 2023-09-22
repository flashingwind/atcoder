use proconio::input;
fn main() {
    input! {
        n: usize,
    };

    input! {
        d: [i32; n],
    };
    let mut c = 0;
    for i in 0..n {
        let mut is_uniq = true;
        for i2 in i + 1..n {
            if d[i] == d[i2] {
                is_uniq = false;
                break;
            }
        }
        if is_uniq {
            //println!("{}: uniq", d[i]);
            c += 1;
        }
    }
    println!("{}", c);
}
