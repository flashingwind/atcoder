use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u32;n],
        mut b: [u32;m],
    };
    a.sort();
    b.sort();
    for i in 0..m {
        if let Ok(idx) = a.binary_search(&b[i]) {
            a.remove(idx);
        } else {
            println!("No");
            return;
        }
        // println!("a {:?}", a);
        // println!("b {:?}", b);
    }
    println!("Yes");
}
