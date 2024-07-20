use proconio::input;

fn main() {
    input! {
        mut h: u32,
    };
    let mut v = 1;
    if h == 0 {
        println!("0");
    }
    for i in 1..33 {
        v *= 2;
        if h <= v {
            println!("{}", i);
            return;
        }
    }
    println!("INF");

    // if h % 2 != 0 {
    //     println!("{}", blen(h) + 1);
    // } else {
    //     println!("{}", blen(h));
    // }
}
// fn blen(v: u32) -> usize {
//     format!("{:b}", v).to_string().len()
// }
