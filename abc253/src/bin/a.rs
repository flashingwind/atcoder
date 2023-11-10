use proconio::input;

fn main() {
    input! {
        a: [u32;3],
    };
    let mut b=a.clone();
    b.sort();
    if a[1]==b[1]{
        println!("Yes");
    }else {
        println!("No")
    }
}
