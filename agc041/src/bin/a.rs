use proconio::input;

fn main() {
    input! {
        n: u128,
        mut a: u128,
        mut b: u128,
    };
    let mut d = b - a;

    if d % 2 == 0 {
        println!("{}", d / 2);
    } else {
        // a→1かb→Nに移動、
        if a <= (n - b) {
            d = a - 1;
            b -= a - 1;
            a = 1;

            d += 1;
            b -= 1;

            d += (b - a) / 2;
        } else {
            d = n - b;
            a += n - b;
            b = n;

            d += 1;
            a += 1;

            d += (b - a) / 2;
        }
        println!("{}", d);
    }
}
