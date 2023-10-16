use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32;n],
    };
    let mut is_same = true;
    for ai in a.iter().skip(1) {
        if *ai != a[0] {
            is_same = false;
        }
    }
    if is_same {
        println!("Yes");
    } else {
        println!("No");
    }
}
