use proconio::input;

fn main() {
    let mut s = vec![];
    loop {
        input! {
            v: u32,
        };
        s.push(v);
        if v == 0 {
            break;
        }
    }
    for &v in s.iter().rev() {
        println!("{}", v);
    }
}
