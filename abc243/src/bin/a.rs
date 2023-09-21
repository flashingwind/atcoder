use proconio::input;

fn main() {
    input! {
        mut v: i32,
        a: i32,
        b: i32,
        c: i32,
    };
    v %= a + b + c;
    // println!("v={v}");
    loop {
        v -= a;
        if v < 0 {
            println!("F");
            break;
        }
        v -= b;
        if v < 0 {
            println!("M");
            break;
        }
        v -= c;
        if v < 0 {
            println!("T");
            break;
        }
    }
}
