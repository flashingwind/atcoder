use proconio::input;

fn main() {
    input! {
        mut a: u64,
        mut b: u64,
        mut c: u64,
    };
    let mut cnt = 0u64;
    while a % 2 == 0 && c % 2 == 0 && c % 2 == 0 {
        let tmp_a = b / 2 + c / 2;
        let tmp_b = a / 2 + c / 2;
        let tmp_c = a / 2 + b / 2;
        a = tmp_a;
        b = tmp_b;
        c = tmp_c;
        if a == b && b == c {
            println!("-1");
            return;
        } else if a % 4 == 1 && b % 4 == 1 && c % 4 == 1 {
            println!("-1");
            return;
        }
        cnt += 1;
    }
    println!("{}", cnt);
}
