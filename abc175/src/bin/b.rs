use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [i64;n],
    };
    let mut cnt = 0;
    for i in 0..n {
        let a = l[i];
        for j in i + 1..n {
            for k in j + 1..n {
                let d1 = (l[j] - l[k]).abs();
                let d2 = l[j] + l[k];
                if d1 == 0 || l[i] == l[j] || l[i] == l[k] {
                    continue;
                }
                if d1 < a && a < d2 {
                    cnt += 1;
                    // println!("{} {} {}: {}<{}<{}", i + 1, j + 1, k + 1, d1, a, d2);
                }
            }
        }
    }
    println!("{cnt}");
}
