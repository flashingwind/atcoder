use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };
    let cs = vec!["0", "2", "4", "6", "8"];
    let mut ans = vec![];
    n -= 1;
    while 0 < n {
        let d = n % 5;
        // println!("{n}%5={d} n/5={}", n / 5);
        n = n / 5;
        ans.insert(0, cs[d]);
        // println!("{:?}={d}", ans);
    }
    if ans.len() == 0 {
        println!("0");
    } else {
        println!("{}", ans.join(""));
    }
}
