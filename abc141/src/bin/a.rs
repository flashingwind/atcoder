use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let weathers = vec!["Sunny", "Cloudy", "Rainy"];
    for (i, _) in weathers.iter().enumerate() {
        if weathers[i] == s {
            println!("{}", weathers[(i + 1) % 3]);
        }
    }
}
