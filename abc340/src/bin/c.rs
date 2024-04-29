use proconio::input;

fn main() {
    input! {
        n: f64,
    };
    let mut nn = vec![(n, 1)];
    let mut c = 0;
    loop {
        while let Some((m, cnt)) = nn.pop() {
            if m % 2.0 == 0 {
                nn.push((m / 2.0, cnt * 2));
                c += 1;
            } else {
                nn.push(((m / 2.0).floor(), cnt));
                nn.push(((m / 2.0).ceil(), cnt));
                c += 1;
            }
        }
        n /= 2;
        for nnn in nn.iter() {
            if nnn < 2 {
                println!("{}", c);
                return;
            }
        }
    }
}
