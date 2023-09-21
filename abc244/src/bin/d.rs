use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s_arr: [char;3],
        t_arr: [char;3],
    };
    let s = s_arr.iter().map(|v| v.to_string()).join("");
    let t = t_arr.iter().map(|v| v.to_string()).join("");
    let pat = [
        String::from("RGB"),
        String::from("GBR"),
        String::from("BRG"),
    ];
    if pat.iter().any(|v| *v == s) == pat.iter().any(|v| *v == t) {
        println!("Yes");
    } else {
        println!("No");
    }
}
