use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [[i32;2];n],
    };
    let mut cnt = 0;
    for dd in d.iter() {
        if dd[0] == dd[1] {
            cnt += 1;
        } else {
            cnt = 0;
        }
        if 3 <= cnt {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
