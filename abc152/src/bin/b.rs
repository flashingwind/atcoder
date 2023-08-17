use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let mut str_ab = a.to_string().repeat(b);
    let mut str_ba = b.to_string().repeat(a);
    if str_ab < str_ba {
        println!("{}", str_ab);
    } else {
        println!("{}", str_ba);
    }
}
