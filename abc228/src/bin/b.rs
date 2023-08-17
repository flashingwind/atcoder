use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: [usize;n],
    };
    a.insert(0, 0);
    let mut cnt = 0;
    let mut is_konwing = vec![false; n + 1];
    let mut i = x;
    loop {
        {
            if is_konwing[i] {
                println!("{}", cnt);
                break;
            }
            cnt += 1;
            is_konwing[i] = true;
            i = a[i];
        }
    }
}
