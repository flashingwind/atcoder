use proconio::input;

fn main() {
    let n = 8;
    input! {
        s: [u64;n],
    };
    for i in 1..n {
        if !(s[i - 1] <= s[i]) {
            // println!("{i} No1: s[i - 1]:{} < s[i]:{}", s[i - 1], s[i]);
            println!("No");
            return;
        }
    }
    for i in 0..n {
        if s[i] < 100 || 675 < s[i] {
            // println!("{i} No2");
            println!("No");
            return;
        }
        if s[i] % 25 != 0 {
            // println!("{i} No3");
            println!("No");
            return;
        }
    }
    println!("Yes");
}
