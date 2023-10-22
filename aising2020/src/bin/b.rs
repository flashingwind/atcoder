use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
    };
    let mut cnt = 0;
    for (i, &ai) in a.iter().enumerate() {
        if ai % 2 == 1 && i % 2 == 0 {
            cnt += 1;
        }
    }
    println!("{cnt}");
}
