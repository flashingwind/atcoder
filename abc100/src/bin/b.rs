use proconio::input;

fn main() {
    input! {
        d: u32,
        n: u32,
    };
    if n % 100 == 0 {
        let mut div_cnt = 0;
        let mut nn = n;
        while nn % 100 == 0 {
            nn /= 100;
            div_cnt += 1;
        }
        // println!("div_cnt={div_cnt}");
        println!("{}", 100u32.pow(d) * (n + div_cnt));
    } else {
        println!("{}", 100u32.pow(d) * n);
    }
}
