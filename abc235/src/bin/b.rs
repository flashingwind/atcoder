use proconio::input;

fn main() {
    input! {
        n: usize,
        mut hs: [u32;n],
    };
    hs.insert(0, 0);
    let mut hmax_i = 1;
    for (i, _) in hs.iter().enumerate().skip(2) {
        if hs[hmax_i] < hs[i] {
            hmax_i = i;
        } else {
            break;
        }
    }
    println!("{}", hs[hmax_i]);
}
