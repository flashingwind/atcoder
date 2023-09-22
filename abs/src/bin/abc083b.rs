use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    };
    let mut sum = 0;
    for nn in 1..n + 1 {
        let mut dig_sum = 0;
        let mut nnn = nn;
        while 0 < nnn {
            //println!("nnn={}, dig_sum={}", nnn, dig_sum);
            dig_sum += nnn % 10;
            nnn /= 10;
        }
        if a <= dig_sum && dig_sum <= b {
            sum += nn;
            //println!("nn={} dig_sum={} sum={}", nn, dig_sum, sum);
        }
    }
    println!("{}", sum);
}
