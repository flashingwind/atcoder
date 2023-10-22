use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let days = [
        "Monday".to_string(),
        "Tuesday".to_string(),
        "Wednesday".to_string(),
        "Thursday".to_string(),
        "Friday".to_string(),
    ];
    for (i, d) in days.iter().enumerate() {
        if s == *d {
            println!("{}", 5 - i);
        }
    }
}
