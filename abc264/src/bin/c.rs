use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        ha: usize,
        wa: usize,
        a: [[u32;wa];ha],
        hb: usize,
        wb: usize,
        b: [[u32;wb];hb],
    };
    for sel_pat_h in (0..ha).combinations(hb) {
        for sel_pat_w in (0..wa).combinations(wb) {
            if check(&sel_pat_h, &sel_pat_w, &a, &b) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

fn check(
    sel_pat_h: &Vec<usize>,
    sel_pat_w: &Vec<usize>,
    a: &Vec<Vec<u32>>,
    b: &Vec<Vec<u32>>,
) -> bool {
    for i in 0..sel_pat_h.len() {
        for j in 0..sel_pat_w.len() {
            if a[sel_pat_h[i]][sel_pat_w[j]] != b[i][j] {
                return false;
            }
        }
    }
    return true;
}
