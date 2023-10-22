use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        r: usize,
        c: usize,
        // a: [u32;n],
    };
    let mut cnt = 0;
    if c != 1 {
        cnt += 1;
    }
    if c != w {
        cnt += 1;
    }
    if r != 1 {
        cnt += 1;
    }
    if r != h {
        cnt += 1;
    }

    println!("{cnt}");
}
