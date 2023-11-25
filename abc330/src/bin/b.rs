use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i32,
        r: i32,
        a: [i32;n],
    };
    let mut out = vec![];
    for i in 0..n {
        if l <= a[i] && a[i] <= r {
            out.push((a[i]).to_string());
        } else if a[i] < l {
            out.push(l.to_string());
        } else {
            out.push(r.to_string());
        }
    }
    println!("{}", out.join(" "));
}
