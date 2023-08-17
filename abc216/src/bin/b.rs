use proconio::input;

fn main() {
    input! {
        n: usize,
        name: [(String,String);n],
    };
    for i in 0..n {
        for j in i + 1..n {
            if name[i] == name[j] {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
