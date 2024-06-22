use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String;n],
    };
    let mut cnt = 0;
    for i in 0..n {
        if s[i] == "Takahashi".to_string() {
            cnt += 1;
        }
    }
    println!("{cnt}");
}
