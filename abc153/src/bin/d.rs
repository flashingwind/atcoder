use proconio::input;

fn main() {
    input! {
        mut h: u64,
    };
    let mut cnt: u64 = 1;
    while h != 0 {
        h /= 2;
        cnt *= 2;
        // println!("h={h} cnt={cnt}");
    }
    println!("{}", cnt - 1);
}
