use proconio::input;

fn main() {
    input! {
        n: usize,
        twons: [(String,i32);n],
    };

    let mut name = String::new();
    let mut max_p = 0;
    let mut sum_p = 0;
    for (s, p) in twons.iter() {
        if max_p < *p {
            max_p = *p;
            name = s.to_string();
        }
        sum_p += *p;
    }
    if sum_p as f64 / 2f64 < max_p as f64 {
    } else {
        name = String::from("atcoder");
    }
    println!("{}", name);
}
