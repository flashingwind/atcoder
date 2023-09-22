use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut mat = vec![vec![0; n]; n];
    // let mut e = Vec::new();
    // let dp = vec![0; n];
    for i in 0..n - 1 {
        let mut max = 0;
        for j in i + 1..n {
            input! {
                tmp: usize,
            }
            mat[i][j] = tmp;
            mat[j][i] = tmp;
            // es.push((i, j, tmp));
            if max < tmp {
                max = tmp;
            }
        }
    }
    // let visited = Vec::new();
    // for r in mat.iter(){
    //     for v in r.iter(){

    //     }
    // }

    let mut max = 0;
    for pat in (0..n).permutations(n) {
        let mut itr = pat.iter();
        let mut sum = 0;
        loop {
            if let Some(&i) = itr.next() {
                if let Some(&j) = itr.next() {
                    // println!("{i},{j}");
                    sum += mat[i][j];
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        // println!("{:?} sum={sum}", pat);
        if max < sum {
            max = sum;
        }
    }
    println!("{}", max);
}
