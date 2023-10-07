use proconio::input;

fn main() {
    let n = 4;
    input! {
        s: [String;n],
    };
    let p = vec!["H", "2B", "3B", "HR"];
    for i in 0..n {
        if !s.contains(&p[i].to_string()) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
