use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64;n],
    };
    a.sort();

    let mut err = 0;
    let mut last_ai = 400; //last_a i= a[i]
    let mut last_ei = a[n - 1];
    for i in 1..n {
        if 2 <= i && a[i] == last_ai {
            // println!("{},skip:{}", i + 1, last_ei);
        } else {
            last_ei = 0;
            for j in 0..=i - 1 {
                last_ei += (a[i] - a[j]).pow(2);
                // println!("{},{}:{}", i + 1, j + 1, last_ei);
            }
        }
        err += last_ei;
        last_ai = a[i];
    }
    // for i in 1..n {
    //     for j in 0..=i - 1 {
    //         err += (a[i] - a[j]).pow(2);
    //         // println!("{},{}:{err}", i + 1, j + 1);
    //     }
    // }
    println!("{}", err);
}
