use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let mut s2 = String::new();
    for c in s.chars().clone() {
        if let Some(c_prev) = s2.pop() {
            if c_prev != c {
                s2.push(c_prev);
            }
            s2.push(c);
        } else {
            s2.push(c);
        }
    }
    // println!("{:?}", s2);
    if s2 == "ABC" || s2 == "AB" || s2 == "AC" || s2 == "BC" || s2 == "A" || s2 == "B" || s2 == "C"
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
