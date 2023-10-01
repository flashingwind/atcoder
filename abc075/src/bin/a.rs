use proconio::input;

fn main() {
    input! {
        mut a: [i32;3],
    };
    a.sort();
    if a[0] == a[1] {
        println!("{}", a[2]);
    } else {
        println!("{}", a[0]);
    }
}
