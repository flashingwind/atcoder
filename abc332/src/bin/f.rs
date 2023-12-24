use num_rational::Rational64;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64;n],
        lrx: [(Usize1,Usize1,i64);m],
    };
    // println!("{:?}", lrx);
    let mut kitai = vec![Rational64::new_raw(0, 1); n];
    for j in 0..n {
        kitai[j] = Rational64::new_raw(a[j] % 998244353, 1);
    }
    for i in 0..m {
        let l = lrx[i].0;
        let r = lrx[i].1;
        let x = lrx[i].2;
        // println!("{i}: {l},{r} <= {n} : {x}");
        for j in l..=r {
            kitai[j] += Rational64::new_raw(x, (r - l + 1) as i64);
            kitai[j] %= 998244353;
        }
    }
    for j in 0..n {
        if j == 0 {
            print!("{}", kitai[j].to_integer() % 998244353);
        }
        print!(" {}", kitai[j].to_integer() % 998244353);
    }
    println!();
}
