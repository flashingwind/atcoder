use proconio::input;

fn main() {
    input! {
        p: [[i32;2];3],
    };
    let l1 = (p[0][0] - p[1][0]).pow(2) + (p[0][1] - p[1][1]).pow(2);
    let l2 = (p[0][0] - p[2][0]).pow(2) + (p[0][1] - p[2][1]).pow(2);
    let l3 = (p[1][0] - p[2][0]).pow(2) + (p[1][1] - p[2][1]).pow(2);
    // println!("{l1},{l2},{l3}");
    if l1 + l2 == l3 || l1 + l3 == l2 || l2 + l3 == l1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
