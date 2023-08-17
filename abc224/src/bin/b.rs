use proconio::input;

#[allow(clippy::nonminimal_bool)]
fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[u32;w];h],
    };

    for i1 in 0..h {
        for i2 in i1 + 1..h {
            for j1 in 0..w {
                for j2 in j1 + 1..w {
                    // println!("i1={i1},j1={j1} i1={i1},j2={j2}");
                    // println!("i2={i2},j1={j1} i2={i2},j2={j2}");
                    if !(a[i1][j1] + a[i2][j2] <= a[i2][j1] + a[i1][j2]) {
                        // println!(
                        //     "[{i1}][{j1}]:{} + [{i2}][{j2}]:{} > [{i2}][{j1}]:{} + [{i1}][{j2}]:{}",
                        //     a[i1][j1], a[i2][j2], a[i2][j1], a[i1][j2]
                        // );
                        println!("No");
                        return;
                    }
                }
            }
        }
    }
    println!("Yes");
}
