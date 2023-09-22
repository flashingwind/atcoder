use proconio::input;

fn main() {
    input! {
        n: u64,
    };
    let mut s = String::new();
    for i in 0..=n {
        let mut is_found = false;
        for j in 1..=9u64 {
            if n % j == 0 && i % (n / j) == 0 {
                is_found = true;
                s += j.to_string().as_str();
                break;
            }
        }
        if !is_found {
            s += "-";
        }
    }
    println!("{}", s);
}
