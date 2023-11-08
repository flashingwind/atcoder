use proconio::input;

fn main() {
    input! {
        mut a: [i32;3],
    };
    a.sort();
    if (a[1] - a[0]) == (a[2] - a[1]) {
        println!("Yes");
    } else {
        println!("No");
    }
}
